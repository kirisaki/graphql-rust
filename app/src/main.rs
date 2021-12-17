use async_graphql::{Schema, EmptyMutation, EmptySubscription, http::{GraphQLPlaygroundConfig, playground_source},};
use async_graphql_axum::{self, GraphQLRequest, GraphQLResponse};
use axum::{response::{IntoResponse, self}, extract::Extension, Router, AddExtensionLayer, Server, routing::get};
use models::{QueryRoot, UsersSchema, User};
use std::sync::{Arc, Mutex};


#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(Arc::new(Mutex::new(Vec::<User>::new())))
        .finish();

    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .layer(AddExtensionLayer::new(schema));

    println!("Playground: http://localhost:8000");

    Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn graphql_playground() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

async fn graphql_handler(
    schema: Extension<UsersSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
