use serde::Serialize;
use uuid::Uuid;

pub struct IdMaybe {
    pub id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct IdSelect {
    pub id: Uuid,
}
