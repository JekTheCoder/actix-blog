use uuid::Uuid;

pub struct IdReturn {
    pub id: Uuid,
}

pub struct IdMaybe {
    pub id: Option<Uuid>,
}
