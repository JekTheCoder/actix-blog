use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub id: uuid::Uuid,
    pub name: String,
    pub color: String,
    pub category_id: uuid::Uuid,
}
