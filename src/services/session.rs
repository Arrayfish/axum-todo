// 一旦DBアクセスも同じところでいいや
use crate::auth::{AuthSession, Credentials};
use crate::util::app_error::AppError;
use crate::AppState;
use ::entity::{user, user::Entity as User};
use axum::debug_handler;
use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::sync::Arc;

// Command
pub async fn logout(
    State(state): State<Arc<AppState>>,
    mut auth_session: AuthSession,
) -> anyhow::Result<impl IntoResponse, AppError> {
    auth_session.get_user().await?;
    Ok(StatusCode::OK.into_response())
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    mut auth_session: AuthSession,
    Json(payload): Json<Credentials>,
) -> anyhow::Result<Json<user::Model>, AppError> {
    let db = &state.db;
    let user = match auth_session.authenticate(payload.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            anyhow::bail!("Invalid credentials");
        }
        Err(e) => {
            anyhow::bail!("Authentication error: {}", e);
        }
    };
    Ok(Json(user))
}
