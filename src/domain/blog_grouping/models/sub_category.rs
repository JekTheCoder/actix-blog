use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubCategory {
    pub id: uuid::Uuid,
    pub name: String,
    pub category_id: uuid::Uuid,
}
