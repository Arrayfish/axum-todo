// 一旦DBアクセスも同じところでいいや
use ::entity::{todo, todo::Entity as Todo};
use sea_orm::EntityTrait;
use sea_orm::ActiveValue::{Set, NotSet, Unchanged};
use uuid::Uuid;
use axum::extract::{State, Path, Json};
use std::sync::Arc;
use crate::AppState;
use serde::Deserialize;
use crate::util::app_error::AppError;

// Query
pub async fn get_user_todos(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<Uuid>, // TODO: 後で直す
) -> anyhow::Result<Json<Vec<todo::Model>>, AppError> {
    let db = &state.db;
    let todo_list = Todo::find_by_id(user_id)
        .all(db).await?;
    Ok(Json(todo_list))
}

// Command
#[derive(Deserialize)]
struct CreateTodo{
    user_id: Uuid,
    content: String,
}

// pub async fn create_todo(
//     State(state): State<Arc<AppState>>,
//     Json(payload): Json<CreateTodo>,
// ) -> anyhow::Result<todo::Model> {
//     let db = &state.db;
//     let todo = todo::ActiveModel {
//         id: Set(Uuid::new_v4()),
//         user_id: Set(user_id),
//         content: Set(content),
//         done: Set(false),
//         ..Default::default()
//     };
//     let todo: todo::Model = todo.insert(db).await?;
//     Ok(Json(todo))
// }


// #[derive(Deserialize)]
// struct UpdateTodo{
//     content: Option<String>,
//     done: Option<bool>,
// }

// pub async fn update_todo(
//     State(state): State<Arc<AppState>>,
//     Path(todo_id): Path<Uuid>,
//     Json(payload): Json<UpdateTodo>,
// ) -> anyhow::Result<todo::Model> {
//     let db = &state.db;
//     let todo: Option<todo::Model> = Todo::find_by_id(todo_id)
//         .one(db).await?;
//     let mut todo: todo::ActiveModel = todo.unwrap().into();
//     match payload.content {
//         Some(content) => todo.content = Set(content),
//         None => (),
//     }
//     match payload.done {
//         Some(done) => todo.done = Set(done),
//         None => (),
//     }

//     let todo: todo::Model = todo.update(db).await?;
//     Ok(todo)
// }

// pub async fn delete_todo(
//     State(state): State<Arc<AppState>>,
//     todo_id: Uuid,
// ) -> anyhow::Result<()> {
//     let db = &state.db;
//     let todo: Option<todo::Model> = Todo::find_by_id(todo_id)
//         .one(db).await?;
//     todo.unwrap().delete(db).await?;
//     Ok(())
// }