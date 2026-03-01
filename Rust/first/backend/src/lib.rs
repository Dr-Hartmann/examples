use dotenvy::dotenv;
use std::env;

pub use poem;
pub use axum;

// http://127.0.0.1:3000
// http://127.0.0.1:3000/docs


pub fn start() {
    dotenv().ok();
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD должна быть установлена!");
    let db_user = env::var("DB_USER").expect("DB_USER должна быть установлена!");
    println!("Пароль от базы данных: {}", db_password);
    println!("Пользователь базы данных: {db_user}");
}
