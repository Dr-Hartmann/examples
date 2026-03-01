use poem::listener::TcpListener;
use poem::{Route, Server};
use poem_openapi::{OpenApi, OpenApiService, payload::PlainText};

#[tokio::main]
pub async fn start() {
    let api_service =
        OpenApiService::new(Api, "Hello World", "1.0").server("http://localhost:3000");

    let ui = api_service.swagger_ui();

    let app = Route::new().nest("/", api_service).nest("/docs", ui);

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
        .unwrap();
}

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/hello/word", method = "get")]
    async fn index(&self) -> PlainText<&'static str> {
        PlainText("Hello World")
    }
}
