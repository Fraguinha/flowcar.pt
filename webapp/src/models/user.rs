use std::collections::HashSet;

#[cfg(feature = "ssr")]
use axum::async_trait;
#[cfg(feature = "ssr")]
use axum_session_auth::{Authentication, HasPermission};
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::PgPool;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub permissions: HashSet<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserPasshash(pub String);

#[cfg(feature = "ssr")]
#[derive(sqlx::FromRow, Clone)]
pub struct SqlUser {
    pub id: i64,
    pub username: String,
    pub password: String,
}

#[cfg(feature = "ssr")]
#[derive(sqlx::FromRow, Clone)]
pub struct SqlPermissionTokens {
    pub token: String,
}

#[cfg(feature = "ssr")]
impl User {
    pub async fn get(id: i64, pool: &PgPool) -> Option<Self> {
        User::get_with_passhash(id, pool)
            .await
            .map(|(user, _)| user)
    }

    pub async fn get_with_passhash(id: i64, pool: &PgPool) -> Option<(Self, UserPasshash)> {
        let sqluser = sqlx::query_as::<_, SqlUser>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
            .ok()?;

        let sql_user_perms = sqlx::query_as::<_, SqlPermissionTokens>(
            "SELECT token FROM permission_tokens WHERE user_id = $1",
        )
        .bind(id)
        .fetch_all(pool)
        .await
        .ok()?;

        Some(sqluser.into_user(Some(sql_user_perms)))
    }

    pub async fn get_from_username(name: String, pool: &PgPool) -> Option<Self> {
        User::get_from_username_with_passhash(name, pool)
            .await
            .map(|(user, _)| user)
    }

    pub async fn get_from_username_with_passhash(
        name: String,
        pool: &PgPool,
    ) -> Option<(Self, UserPasshash)> {
        let sqluser = sqlx::query_as::<_, SqlUser>("SELECT * FROM users WHERE username = $1")
            .bind(name)
            .fetch_one(pool)
            .await
            .ok()?;

        let sql_user_perms = sqlx::query_as::<_, SqlPermissionTokens>(
            "SELECT token FROM permission_tokens WHERE user_id = $1",
        )
        .bind(sqluser.id)
        .fetch_all(pool)
        .await
        .ok()?;

        Some(sqluser.into_user(Some(sql_user_perms)))
    }
}

#[cfg(feature = "ssr")]
impl SqlUser {
    pub fn into_user(
        self,
        sql_user_perms: Option<Vec<SqlPermissionTokens>>,
    ) -> (User, UserPasshash) {
        (
            User {
                id: self.id,
                username: self.username,
                permissions: if let Some(user_perms) = sql_user_perms {
                    user_perms
                        .into_iter()
                        .map(|x| x.token)
                        .collect::<HashSet<String>>()
                } else {
                    HashSet::<String>::new()
                },
            },
            UserPasshash(self.password),
        )
    }
}

#[cfg(feature = "ssr")]
#[async_trait]
impl Authentication<User, i64, PgPool> for User {
    async fn load_user(userid: i64, pool: Option<&PgPool>) -> Result<User, anyhow::Error> {
        let pool = pool.unwrap();

        Ok(User::get(userid, pool).await.unwrap())
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    fn is_active(&self) -> bool {
        true
    }

    fn is_anonymous(&self) -> bool {
        false
    }
}

#[cfg(feature = "ssr")]
#[async_trait]
impl HasPermission<PgPool> for User {
    async fn has(&self, perm: &str, _pool: &Option<&PgPool>) -> bool {
        self.permissions.contains(perm)
    }
}
