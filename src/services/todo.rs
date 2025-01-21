// 一旦DBアクセスも同じところでいいや
use ::entity::{todo, todo::Entity as Todo};
use sea_orm::*;
use sea_orm::ActiveValue::{Set, NotSet, Unchanged};
use uuid::Uuid;
use axum::extract::{State, Path, Json};
use std::sync::Arc;
use crate::AppState;
use serde::Deserialize;
use crate::util::app_error::AppError;

// Query
pub async fn get_all_todos(
    State(state): State<Arc<AppState>>,
) -> anyhow::Result<Json<Vec<todo::Model>>, AppError> {
    println!("get_all_todos called");
    let db = &state.db;
    let todo_list = Todo::find().all(db).await?;
    println!("{:?}", todo_list);
    Ok(Json(todo_list))
}

pub async fn get_user_todos(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<Uuid>, // TODO: 後で直す
) -> anyhow::Result<Json<Vec<todo::Model>>, AppError> {
    println!("get_user_todos called");
    let db = &state.db;
    let todo_list = Todo::find().filter(todo::Column::UserId.eq(user_id))
        .all(db).await?;
    println!("{:?}", todo_list);
    Ok(Json(todo_list))
}

// Command
#[derive(Deserialize)]
pub struct CreateTodo{
    user_id: Uuid,
    content: String,
}
pub async fn create_todo(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTodo>,
) -> anyhow::Result<Json<todo::Model>, AppError> {
    let db = &state.db;
    let todo = todo::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(payload.user_id),
        content: Set(payload.content),
        done: Set(false),
        ..Default::default()
    };
    let todo: todo::Model = todo.insert(db).await?;
    Ok(Json(todo))
}


#[derive(Deserialize)]
pub struct UpdateTodo{
    content: Option<String>,
    done: Option<bool>,
}
pub async fn update_todo(
    State(state): State<Arc<AppState>>,
    Path(todo_id): Path<Uuid>,
    Json(payload): Json<UpdateTodo>,
) -> anyhow::Result<Json<todo::Model>, AppError> {
    let db = &state.db;
    let todo: Option<todo::Model> = Todo::find_by_id(todo_id)
        .one(db).await?;
    let mut todo: todo::ActiveModel = todo.unwrap().into();
    match payload.content {
        Some(content) => todo.content = Set(content),
        None => (),
    }
    match payload.done {
        Some(done) => todo.done = Set(done),
        None => (),
    }

    let todo: todo::Model = todo.update(db).await?;
    Ok(Json(todo))
}

pub async fn delete_todo(
    State(state): State<Arc<AppState>>,
    Path(todo_id): Path<Uuid>,
) -> anyhow::Result<(), AppError> {
    let db = &state.db;
    let todo: Option<todo::Model> = Todo::find_by_id(todo_id)
        .one(db).await?;
    todo.unwrap().delete(db).await?;
    Ok(())
}