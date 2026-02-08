// basic handler that responds with a static string
// #[utoipa::path(
//     get,
//     path = "/hello",
//     responses(
//         (status = 200, description = "Приветствие успешно получено", body = String)        ,
//      (status = NOT_FOUND, description = "Pet was not found")
//     ),
// params(
//    ("id" = u64, Path, description = "Pet database id to get Pet for"),
//  )
// )]
pub async fn root() -> &'static str {
    "Hello, World!"
}

use dto::CreateUser;
use entity::User;
pub async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };
    (StatusCode::CREATED, Json(user))
}

use serde_json::{Value, json};
pub async fn json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

use axum::{Json, extract::Query, http::StatusCode, response::IntoResponse};
use std::collections::HashMap;
pub async fn echo(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let response = format!("Получены параметры: {:?}", params);
    (StatusCode::OK, response)
}
