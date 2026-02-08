use axum::{
    Router,
    routing::{get, post},
};

use service::{create_user, echo, json, root};

// #[derive(Clone)]
// struct AppState {}

// let state = AppState {};

#[tokio::main]
async fn main() {
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

// use utoipa::OpenApi;
// use utoipa_swagger_ui::SwaggerUi;

// #[derive(OpenApi)]
// #[openapi(
//     paths(hello_world),
//     tags((name = "hello-api", description = "Тестовое API"))
// )]
// struct ApiDoc;
// println!("{}", ApiDoc::openapi().to_pretty_json().unwrap());
