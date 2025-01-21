use axum::{
    routing::{get, post, put, delete},
    Router, 
};
use sea_orm::{Database, DatabaseConnection};
use std::net::SocketAddr;
use tokio::net::TcpListener;
mod services;
use services::{todo, user};
use std::sync::Arc;
mod util;
struct AppState{
    db: DatabaseConnection
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let shared_state = Arc::new(AppState{
        db: Database::connect("postgresql://root:password@localhost:5433/postgres").await?
    });
    println!("Connected to database");
    // pass incoming GET requests on "/hello-world" to "hello_world" handler.
    let app = Router::new()
        .route("/hello-world", get(hello_world))
        .route("/todos/{user_id}", get(todo::get_user_todos))
        .route("/user", post(user::create_user))
        .route("/todo", 
            get(todo::get_all_todos)
            .post(todo::create_todo)
        )
        .route("/todo/{todo_id}", 
            put(todo::update_todo)
            .delete(todo::delete_todo))
        .with_state(shared_state);

    // write address like this to not make typos
    let addr = SocketAddr::from(([127, 0, 0, 1], 3005));
    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}
