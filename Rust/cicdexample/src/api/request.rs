use poem_openapi::Object;
use serde::Deserialize;

#[derive(Debug, Deserialize, Object)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub age: u8,
}
