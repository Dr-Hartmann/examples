use crate::dto::UserSchema;
use poem_openapi::{ApiResponse, payload::Json};

#[derive(ApiResponse)]
pub enum GetUserResponse {
    #[oai(status = 200)]
    Ok(Json<UserSchema>),
    #[oai(status = 404)]
    NotFound,
}

#[derive(ApiResponse)]
pub enum CreateUserResponse {
    #[oai(status = 201)]
    Created(Json<UserSchema>),
    #[oai(status = 400)]
    BadRequest(Json<String>),
}

#[derive(ApiResponse)]
pub enum GetUsersResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<UserSchema>>),
}

#[derive(ApiResponse)]
pub enum UpdateUserResponse {
    #[oai(status = 200)]
    Ok(Json<UserSchema>),
    #[oai(status = 404)]
    NotFound,
}

#[derive(ApiResponse)]
pub enum DeleteUserResponse {
    #[oai(status = 204)]
    Deleted,
    #[oai(status = 404)]
    NotFound,
}
