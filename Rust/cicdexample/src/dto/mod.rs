use crate::domain::User;
use bon::Builder;
use chrono::{DateTime, Utc};
use poem_openapi::Object;
use serde::Serialize;
use uuid::Uuid;

#[derive(Builder, Debug, Object, Serialize, Clone)]
pub struct UserSchema {
    pub id: Option<Uuid>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub age: Option<u8>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<User> for UserSchema {
    fn from(u: User) -> Self {
        Self::builder()
            .id(u.id)
            .username(u.username)
            .email(u.email)
            .age(u.age)
            .created_at(u.created_at)
            .updated_at(u.updated_at)
            .build()
    }
}
