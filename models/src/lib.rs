use std::sync::{Arc, Mutex};

use async_graphql::{Object, Context, Schema, EmptyMutation, EmptySubscription};


pub type UsersSchema= Schema<QueryRoot, EmptyMutation, EmptySubscription>;

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


