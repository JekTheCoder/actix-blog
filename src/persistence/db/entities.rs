mod insert;
mod select;

pub use id_select::IdSelect;
pub use insert::InsertErr;
pub use select::SelectErr;

mod id_select {
    use serde::Serialize;
    use uuid::Uuid;

    #[derive(Serialize)]
    pub struct IdSelect {
        pub id: Uuid,
    }
}
