use poem::{Route, Server, listener::TcpListener};
use poem_openapi::OpenApiService;

mod api;
mod domain;
mod dto;
mod repository;
mod service;

use api::Api;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api_service = OpenApiService::new(Api::new(), "Простой пример API", "1.0")
        .server("http://localhost:3000");

    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/", api_service).nest("/docs", ui);

    let listener = TcpListener::bind("127.0.0.1:3000");
    Server::new(listener).run(app).await
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     use chrono::{DateTime, Utc};
//     use uuid::Uuid;

//     #[test]
//     fn test_user_builder_full() {
//         let custom_id = Uuid::new_v4();
//         let custom_time = Utc::now();

//         // Проверяем явную передачу всех полей
//         let user = User::builder()
//             .id(custom_id)
//             .username("ivan") // String через Into
//             .email("ivan@test.com")
//             .age(30)
//             .created_at(custom_time)
//             .build();

//         assert_eq!(user.id, custom_id);
//         assert_eq!(user.username, "ivan");
//         assert_eq!(user.age, 30);
//         assert_eq!(user.created_at, custom_time);
//     }

//     #[test]
//     fn test_user_builder_defaults() {
//         // Проверяем работу #[builder(default)]
//         let user = User::builder()
//             .username("alice")
//             .email("alice@test.com")
//             .age(25)
//             .build();

//         // ID должен быть сгенерирован автоматически
//         assert!(!user.id.is_nil());

//         // Время должно быть установлено автоматически (не в будущем)
//         assert!(user.created_at <= Utc::now());

//         assert_eq!(user.username, "alice");
//     }

//     #[test]
//     fn test_user_clone_and_debug() {
//         let user = User::builder()
//             .username("bob")
//             .email("bob@test.com")
//             .age(40)
//             .build();

//         let cloned = user.clone();
//         assert_eq!(user.id, cloned.id);

//         // Проверка наличия Debug (чтобы код компилировался)
//         let debug_str = format!("{:?}", user);
//         assert!(debug_str.contains("bob"));
//     }
// }
