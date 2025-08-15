use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use sqlx::{FromRow, types::Uuid};

use crate::application::roles;

#[derive(Debug, FromRow, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub password_salt: String,
    pub active: bool,
    pub roles: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>
}

impl User {
    pub fn is_admin(&self) -> bool {
        roles::containers_role_admin(&self.roles)
    }
}