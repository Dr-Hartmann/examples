use bon::Builder;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Builder, Debug, Clone)]
pub struct User {
    #[builder(default = Uuid::new_v4())]
    pub id: Uuid,
    #[builder(into)]
    pub username: String,
    #[builder(into)]
    pub email: String,
    pub age: u8,
    #[builder(default = Utc::now())]
    pub created_at: DateTime<Utc>,
    #[builder(default = Utc::now())]
    pub updated_at: DateTime<Utc>,
}
