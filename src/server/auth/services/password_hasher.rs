use crate::server::{auth::models::hashed_password::HashedPassword, service::sync_service};

sync_service!(PasswordHasher;);

impl PasswordHasher {
    pub fn hash(&self, password: impl AsRef<str>) -> HashedPassword {
        let password = password.as_ref();
        let password = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();

        HashedPassword::new_unchecked(password)
    }
}
