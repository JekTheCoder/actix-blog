use actix_web::web::Data;
use futures_util::StreamExt;
use markdown_parse::BlogParse;
use par_stream::ParStreamExt;
use uuid::Uuid;

use crate::{
    domain::blog::ImgHostInjectorFactory, persistence::db::Pool, server::service::sync_service,
};

sync_service!(RecompileMarkdowns; pool: Data<Pool>, injector_factory: ImgHostInjectorFactory);

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
    pub async fn run(&self) -> Result<(), sqlx::Error> {
        use static_pool::*;

        let blogs = sqlx::query_as!(BlogContent, "SELECT id, content FROM blogs")
            .fetch(StaticPool::from(self.pool.clone()));

        let outer_pool = self.pool.clone();
        let outer_factory = self.injector_factory.clone();

        blogs
            .par_then(None, move |blog| {
                let pool = outer_pool.clone();
                let factory = outer_factory.clone();

                update_blog(blog, pool, factory)
            })
            .collect::<Vec<_>>()
            .await;

        Ok(())
    }
}

async fn update_blog(
    blog: Result<BlogContent, sqlx::error::Error>,
    pool: Data<Pool>,
    factory: ImgHostInjectorFactory,
) {
    if let Ok(BlogContent { id, content }) = blog {
        let injector = factory.create(id);

        let BlogParse {
            content: html_content,
            ..
        } = markdown_parse::parse(content.as_str(), &injector).unwrap();

        let _ = sqlx::query!(
            r#"UPDATE blogs SET html = $1 WHERE id = $2"#,
            &html_content,
            id
        )
        .execute(pool.as_ref())
        .await
        .unwrap();
    }
}
