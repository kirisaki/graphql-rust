use std::sync::{Arc, Mutex};

use async_graphql::{Object, Context, Schema, EmptySubscription, Result};


pub type UsersSchema= Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[derive(Clone)]
pub struct User {
    id: String,
    name: String,
    note: Option<String>,
}

#[Object]
impl User {
    async fn id(&self) -> &str {
        &self.id
    }
    async fn name(&self) -> &str {
        &self.name
    }
    async fn note(&self) -> Option<&str> {
        match &self.note {
            Some(x) => Some(&x),
            None => None
        }
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn users(&self, ctx: &Context<'_>) -> Vec<User> {
        ctx
            .data_unchecked::<Arc<Mutex<Vec<User>>>>()
            .lock()
            .unwrap()
            .iter()
            .cloned()
            .collect()
    }

    async fn user_by_id(&self, ctx: &Context<'_>, id: String) -> Option<User> {
        ctx
            .data_unchecked::<Arc<Mutex<Vec<User>>>>()
            .lock()
            .unwrap()
            .iter()
            .cloned()
            .find(|x| x.id == id)
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn add_user(&self, ctx: &Context<'_>, id: String, name: String) -> Result<String> {
        let id0 = id.clone();
        let user = User{id, name, note: None};
        let mut users = ctx
            .data_unchecked::<Arc<Mutex<Vec<User>>>>()
            .lock()
            .unwrap();
        users.push(user);
        Ok(id0)
    }
}


