pub use id_select::IdSelect;

mod id_select {
    use serde::Serialize;
    use uuid::Uuid;

    #[derive(Serialize)]
    pub struct IdSelect {
        pub id: Uuid,
    }
}
