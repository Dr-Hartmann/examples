use axum::{
    Router,
    routing::{get, post},
};

#[tokio::main]
pub async fn start() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/json", get(json))
        .route("/users", post(create_user))
        .route("/echo", get(echo))
    // .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
    // .with_state(state)
;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

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

use api::request::CreateUser;
use api::response::User;
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
