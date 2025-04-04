use ::entity::{user, user::Entity as User};
use axum_login::{AuthUser, AuthnBackend, UserId};
use password_auth::verify_password;
use sea_orm::prelude::*;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use tokio::task;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserWrapper(user::Model);

impl Deref for UserWrapper {
    type Target = user::Model;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AuthUser for UserWrapper {
    type Id = Uuid;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Credentials {
    user_id: Uuid,
    password: String,
}

#[derive(Clone, Debug)]
struct Backend {
    db: DatabaseConnection,
}

impl Backend {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

impl AuthnBackend for Backend {
    type User = UserWrapper;
    type Credentials = Credentials;
    type Error = anyhow::Error;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<user::Model>, Self::Error> {
        let user = user::Entity::find_by_id(creds.user_id).one(self.db).await?;

        task::spawn_blocking(|| {
            Ok(user.filter(|user| verify_password(creds.password, &user.password).is_ok()))
        })
        .await?
    }

    async fn get_user(&self, user_id: UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user = user::Entity::find_by_id(user_id).one(self.db).await?;
        Ok(user)
    }
}

pub type AuthSession = axum_login::AuthSession<Backend>;
