use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Tag {
    pub id: uuid::Uuid,
    pub name: String,
    pub color: String,
    pub category_id: uuid::Uuid,
}
