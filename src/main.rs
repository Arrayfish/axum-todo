use axum::{
    routing::get, 
    Router, 
    response::Json
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use serde_json::{Value, json};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // pass incoming GET requests on "/hello-world" to "hello_world" handler.
    let app = Router::new()
        .route("/hello-world", get(hello_world))
        .route("/user", get(user).post(user))
        .route("/todo", get(todo).post(todo))
        .route("/todo/:id", put(todo).delete(todo));

    // write address like this to not make typos
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn todo() -> Json<Value> {
    Json(json!([
        {
            "content": "clean my room",
            "done": false
        },
        {
            "content": "Bye the apple",
            "done": false
        },] 
    ))
}