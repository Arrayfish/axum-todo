// 一旦DBアクセスも同じところでいいや
use ::entity::{user, user::Entity as User};
use sea_orm::ActiveValue::{Set, NotSet, Unchanged};
use uuid::Uuid;
use axum::extract::State;
use std::sync::Arc;

use crate::AppState;
// Command
// pub async fn create_user(
//     State(state): State<Arc<AppState>>, 
//     name: String,
//     email: String,
//     password: String,
// ) -> anyhow::Result<user::Model> {
//     let db = &state.db;
//     let user = user::ActiveModel {
//         id: Set(Uuid::new_v4()),
//         name: Set(name),
//         email: Set(email),
//         password: Set(password),
//         ..Default::default()
//     };
//     let user: user::Model = user.insert(db).await?;
//     Ok(user)
// }

// pub async fn update_user(
//     State(state): State<Arc<AppState>>,
//     user_id: Uuid,
//     name: Option<String>,
//     email: Option<String>,
//     password: Option<String>,
// ) -> anyhow::Result<user::Model> {
//     let db = &state.db;
//     let user: Option<user::Model> = User::find_by_id(user_id)
//         .one(db).await?;
//     let mut user: user::ActiveModel = user.unwrap().into();
//     match name {
//         Some(name) => user.name = Set(name),
//         None => (),
//     }
//     match email {
//         Some(email) => user.email = Set(email),
//         None => (),
//     }
//     match password {
//         Some(password) => user.password = Set(password),
//         None => (),
//     }

//     let user: user::Model = user.update(db).await?;
//     Ok(user)
// }

// pub async fn delete_user(
//     State(state): State<Arc<AppState>>,
//     user_id: Uuid,
// ) -> anyhow::Result<()> {
//     let db = &state.db;
//     let user: Option<user::Model> = User::find_by_id(user_id)
//         .one(db).await?;
//     user.unwrap().delete(db).await?;
//     Ok(())
// }