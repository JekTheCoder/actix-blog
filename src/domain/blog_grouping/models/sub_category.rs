use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct SubCategory {
    pub id: uuid::Uuid,
    pub name: String,
    pub category_id: uuid::Uuid,
}
