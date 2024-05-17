use actix_web::web::Data;
use futures_util::StreamExt;
use markdown_parse::content::ContentBuf;
use par_stream::ParStreamExt;
use uuid::Uuid;

use crate::{persistence::db::Pool, server::service::sync_service};

use super::set_content::SetContent;

sync_service!(RecompileMarkdowns; pool: Data<Pool>, set_content: SetContent);

struct BlogContent {
    pub id: Uuid,
    pub content: String,
}

mod static_pool {
    use actix_web::web::Data;

    use crate::persistence::db::{Database, Pool};

    #[derive(Debug)]
    pub struct StaticPool(Data<Pool>);

    impl From<Data<Pool>> for StaticPool {
        fn from(pool: Data<Pool>) -> Self {
            Self(pool)
        }
    }

    impl sqlx::Executor<'static> for StaticPool {
        type Database = Database;

        fn describe<'e, 'q: 'e>(
            self,
            sql: &'q str,
        ) -> futures_util::future::BoxFuture<'e, Result<sqlx::Describe<Self::Database>, sqlx::Error>>
        where
            'static: 'e,
        {
            self.0.as_ref().describe(sql)
        }

        fn fetch_many<'e, 'q: 'e, E: 'q>(
            self,
            query: E,
        ) -> futures_util::stream::BoxStream<
            'e,
            Result<
                sqlx::Either<
                    <Self::Database as sqlx::Database>::QueryResult,
                    <Self::Database as sqlx::Database>::Row,
                >,
                sqlx::Error,
            >,
        >
        where
            'static: 'e,
            E: sqlx::Execute<'q, Self::Database>,
        {
            self.0.as_ref().fetch_many(query)
        }

        fn fetch_optional<'e, 'q: 'e, E: 'q>(
            self,
            query: E,
        ) -> futures_util::future::BoxFuture<
            'e,
            Result<Option<<Self::Database as sqlx::Database>::Row>, sqlx::Error>,
        >
        where
            'static: 'e,
            E: sqlx::Execute<'q, Self::Database>,
        {
            self.0.as_ref().fetch_optional(query)
        }

        fn prepare_with<'e, 'q: 'e>(
            self,
            sql: &'q str,
            parameters: &'e [<Self::Database as sqlx::Database>::TypeInfo],
        ) -> futures_util::future::BoxFuture<
            'e,
            Result<<Self::Database as sqlx::database::HasStatement<'q>>::Statement, sqlx::Error>,
        >
        where
            'static: 'e,
        {
            self.0.as_ref().prepare_with(sql, parameters)
        }
    }
}

impl RecompileMarkdowns {
    pub async fn run(self) -> Result<(), sqlx::Error> {
        let blogs = sqlx::query_as!(BlogContent, "SELECT id, content FROM blogs")
            .fetch(static_pool::StaticPool::from(self.pool));

        blogs
            .par_then(None, move |blog| {
                let set_content = self.set_content.clone();

                async move {
                    let Ok(BlogContent { id, content }) = blog else {
                        return;
                    };

                    // Is always valid because it is stored
                    let content = ContentBuf::from_boxed_unchecked(content.into_boxed_str());

                    if let Err(e) = set_content
                        .run(id, &content, /* Force to recompile preview */ None)
                        .await
                    {
                        eprintln!("Got error while recompiling markdown: {:?}", e);
                    }
                }
            })
            .collect::<Vec<_>>()
            .await;

        Ok(())
    }
}
