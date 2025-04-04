// 一旦DBアクセスも同じところでいいや
use crate::util::app_error::AppError;
use ::entity::{user, user::Entity as User};
use axum::extract::{Json, State};
use sea_orm::ActiveValue::{NotSet, Set, Unchanged};
use sea_orm::*;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::AppState;
// Command
#[derive(Deserialize)]
pub struct CreateUser {
    name: String,
    email: String,
    password: String,
}
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUser>,
) -> anyhow::Result<Json<user::Model>, AppError> {
    let db = &state.db;
    let user = user::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(payload.name),
        email: Set(payload.email),
        password: Set(payload.password),
        ..Default::default()
    };
    let user: user::Model = user.insert(db).await?;
    Ok(Json(user))
}

#[derive(Deserialize)]
pub struct UpdateUser {
    user_id: Uuid,
    name: Option<String>,
    email: Option<String>,
    password: Option<String>,
}
pub async fn update_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateUser>,
) -> anyhow::Result<Json<user::Model>> {
    let db = &state.db;
    let user: Option<user::Model> = User::find_by_id(payload.user_id).one(db).await?;
    let mut user: user::ActiveModel = user.unwrap().into();
    match payload.name {
        Some(name) => user.name = Set(name),
        None => (),
    }
    match payload.email {
        Some(email) => user.email = Set(email),
        None => (),
    }
    match payload.password {
        Some(password) => user.password = Set(password),
        None => (),
    }

    let user: user::Model = user.update(db).await?;
    Ok(Json(user))
}

#[derive(Deserialize)]
pub struct DeleteUser {
    user_id: Uuid,
}
pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<DeleteUser>,
) -> anyhow::Result<()> {
    let db = &state.db;
    let user: Option<user::Model> = User::find_by_id(payload.user_id).one(db).await?;
    user.unwrap().delete(db).await?;
    Ok(())
}
