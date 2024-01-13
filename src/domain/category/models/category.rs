use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Category {
    pub id: uuid::Uuid,
    pub name: String,
}
