use crate::domain::User;
use crate::dto::UserSchema;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(Clone, Default)]
pub struct Database {
    pub inner: Arc<RwLock<HashMap<Uuid, User>>>,
}

impl Database {
    pub fn create(&self, item: UserSchema) -> User {
        self.insert(
            User::builder()
                .username(item.username.unwrap_or_default())
                .email(item.email.unwrap_or_default())
                .age(item.age.unwrap_or(0))
                .build(),
        )
    }

    pub fn list(&self) -> Vec<User> {
        self.inner.read().unwrap().values().cloned().collect() // TODO
    }

    pub fn delete(&self, id: Uuid) -> bool {
        self.inner.write().unwrap().remove(&id).is_some() // TODO
    }

    pub fn update(&self, id: Uuid, item: UserSchema) {
        // TODO
        self.inner.write().unwrap().insert(
            id,
            User::builder()
                .username(item.username.unwrap_or_default())
                .email(item.email.unwrap_or_default())
                .age(item.age.unwrap_or_default())
                .created_at(item.created_at.unwrap_or_default())
                .build(),
        );
    }

    pub fn insert(&self, user: User) -> User {
        self.inner.write().unwrap().insert(user.id, user.clone()); // TODO
        user
    }

    pub fn get(&self, id: Uuid) -> Option<User> {
        self.inner.read().unwrap().get(&id).cloned() // TODO
    }
}
