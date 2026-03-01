use crate::domain::User;
use crate::repository::Database;

pub fn init() -> Database {
    let db = Database::default();
    db.insert(
        User::builder()
            .username("alice")
            .email("alice@mail.com")
            .age(28)
            .build(),
    );
    db.insert(
        User::builder()
            .username("bob")
            .email("bob@mail.com")
            .age(34)
            .build(),
    );
    db
}
