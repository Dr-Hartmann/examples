mod request;
mod response;

use crate::{dto::UserSchema, repository::Database};
use poem_openapi::{OpenApi, param::Path, payload::Json};
use uuid::Uuid;

pub struct Api {
    db: Database,
}

impl Api {
    pub fn new() -> Self {
        Self {
            db: crate::service::init(),
        }
    }
}

use request::CreateUserRequest;
use response::{
    CreateUserResponse, DeleteUserResponse, GetUserResponse, GetUsersResponse, UpdateUserResponse,
};
#[OpenApi]
impl Api {
    #[oai(path = "/users/:id", method = "get")]
    async fn get_user(&self, id: Path<Uuid>) -> GetUserResponse {
        match self.db.get(id.0) {
            Some(user) => GetUserResponse::Ok(Json(user.into())),
            None => GetUserResponse::NotFound,
        }
    }

    #[oai(path = "/users", method = "post")]
    async fn create_user(&self, data: Json<CreateUserRequest>) -> CreateUserResponse {
        let item = self.db.create(
            UserSchema::builder()
                .username(data.0.username)
                .email(data.0.email)
                .age(data.0.age)
                .build(),
        );
        CreateUserResponse::Created(Json(item.into()))
    }

    #[oai(path = "/users", method = "get")]
    async fn get_users(&self) -> GetUsersResponse {
        let users = self.db.list();
        let schemas: Vec<UserSchema> = users.into_iter().map(UserSchema::from).collect();
        GetUsersResponse::Ok(Json(schemas))
    }

    #[oai(path = "/users/:id", method = "put")]
    async fn update_user(
        &self,
        id: Path<Uuid>,
        data: Json<CreateUserRequest>,
    ) -> UpdateUserResponse {
        let user_id = id.0;
        if let Some(v) = self.db.get(user_id) {
            let updated = UserSchema::builder()
                .username(data.0.username)
                .email(data.0.email)
                .age(data.0.age)
                .created_at(v.created_at)
                .build();

            self.db.update(user_id, updated.clone());
            UpdateUserResponse::Ok(Json(updated.into()))
        } else {
            UpdateUserResponse::NotFound
        }
    }

    #[oai(path = "/users/:id", method = "delete")]
    async fn delete_user(&self, id: Path<Uuid>) -> DeleteUserResponse {
        if self.db.delete(id.0) {
            DeleteUserResponse::Deleted
        } else {
            DeleteUserResponse::NotFound
        }
    }
}
