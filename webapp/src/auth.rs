#[cfg(feature = "ssr")]
use crate::database::db;
use crate::models::user::User;
#[cfg(feature = "ssr")]
use crate::models::user::UserPasshash;
#[cfg(feature = "ssr")]
use axum_session_auth::AuthSession;
#[cfg(feature = "ssr")]
use axum_session_sqlx::SessionPgPool;
#[cfg(feature = "ssr")]
use bcrypt::{hash, verify, DEFAULT_COST};
#[cfg(feature = "ssr")]
use leptos::use_context;
use leptos::{server, ServerFnError};
#[cfg(feature = "ssr")]
use sqlx::PgPool;

#[cfg(feature = "ssr")]
pub fn auth() -> Result<AuthSession<User, i64, SessionPgPool, PgPool>, ServerFnError> {
    use_context::<AuthSession<User, i64, SessionPgPool, PgPool>>()
        .ok_or_else(|| ServerFnError::ServerError("Auth session missing.".into()))
}

#[server]
pub async fn get_user() -> Result<Option<User>, ServerFnError> {
    let auth = auth()?;

    Ok(auth.current_user)
}

#[server(Login, "/api", "Url", "login")]
pub async fn login(
    username: String,
    password: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    let pool = db().await?;
    let auth = auth()?;

    let (user, UserPasshash(expected_passhash)) =
        User::get_from_username_with_passhash(username, &pool)
            .await
            .ok_or_else(|| ServerFnError::new("Login failed: Username does not exist."))?;

    match verify(password, &expected_passhash)? {
        true => {
            auth.login_user(user.id);
            auth.remember_user(remember.is_some());

            Ok(())
        }
        false => Err(ServerFnError::new("Login failed: Incorrect password.")),
    }
}

#[server(Logout, "/api", "Url", "logout")]
pub async fn logout() -> Result<(), ServerFnError> {
    let auth = auth()?;

    auth.logout_user();

    leptos_axum::redirect("/");

    Ok(())
}

#[server(CreateUser, "/api", "Url", "create-user")]
pub async fn create_user(
    username: String,
    password: String,
    password_confirmation: String,
) -> Result<(), ServerFnError> {
    let pool = db().await?;
    let auth = auth()?;

    if auth.current_user.clone().is_none() {
        return Err(ServerFnError::new("Failed: Requires authentication."));
    }

    if password != password_confirmation {
        return Err(ServerFnError::new("Failed: Passwords did not match."));
    }

    let password_hashed = hash(password, DEFAULT_COST)?;

    sqlx::query("INSERT INTO users (username, password) VALUES ($1, $2)")
        .bind(username.clone())
        .bind(password_hashed)
        .execute(&pool)
        .await?;

    Ok(())
}

#[server(ChangePassUser, "/api", "Url", "update-user")]
pub async fn change_pass_user(
    username: String,
    password: String,
    password_confirmation: String,
) -> Result<(), ServerFnError> {
    let pool = db().await?;
    let auth = auth()?;

    if auth.current_user.clone().is_none() {
        return Err(ServerFnError::new("Failed: Requires authentication."));
    }

    if auth.current_user.clone().unwrap().username != username
        && auth.current_user.clone().unwrap().username != "admin"
    {
        return Err(ServerFnError::new(
            "Failed: Only the user can update his own password.",
        ));
    }

    if password != password_confirmation {
        return Err(ServerFnError::new("Failed: Passwords did not match."));
    }

    let password_hashed = hash(password, DEFAULT_COST)?;

    sqlx::query("UPDATE users SET password = $1 WHERE username = $2")
        .bind(password_hashed)
        .bind(username)
        .execute(&pool)
        .await?;

    Ok(())
}

#[server(DeleteUser, "/api", "Url", "delete-user")]
pub async fn delete_user(username: String) -> Result<(), ServerFnError> {
    let pool = db().await?;
    let auth = auth()?;

    if auth.current_user.clone().is_none() {
        return Err(ServerFnError::new("Failed: Requires authentication."));
    }

    if auth.current_user.clone().unwrap().username != username
        && auth.current_user.clone().unwrap().username != "admin"
    {
        return Err(ServerFnError::new(
            "Failed: Only the user can request his own deletion.",
        ));
    }

    sqlx::query("DELETE FROM users WHERE username = $1 AND id > 0")
        .bind(username)
        .execute(&pool)
        .await?;

    Ok(())
}
