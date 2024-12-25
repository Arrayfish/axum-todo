use axum::{
    routing::{get, post, put, delete},
    Router, 
    response::Json
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use serde_json::{Value, json};
mod services;
use services::{todo, user};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // pass incoming GET requests on "/hello-world" to "hello_world" handler.
    let app = Router::new()
        .route("/hello-world", get(hello_world))
        .route("/user", post(user::create_user(db, name, email, password)))
        .route("/todo", 
            get(todo::get_user_todos(db, user_id))
            .post(todo::create_todo(db, user_id, content))
        )
        .route("/todo/:id", 
            put(todo::update_todo(todo_id, content, done))
            .delete(todo::delete_todo(todo_id)));

    // write address like this to not make typos
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}
