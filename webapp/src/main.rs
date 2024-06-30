use axum::{routing::get, Router};
use axum_session::{SessionConfig, SessionLayer};
use axum_session_auth::{AuthConfig, AuthSessionLayer};
use axum_session_sqlx::{SessionPgPool, SessionPgSessionStore};
use flowcar::app::App;
use flowcar::database::db;
use flowcar::models::user::User;
use flowcar::server::{fallback_handler, routes_handler, server_function_handler};
use flowcar::state::AppState;
use leptos::get_configuration;
use leptos_axum::{generate_route_list, LeptosRoutes};
use sqlx::PgPool;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    let pool = db().await.unwrap();

    let session_config = SessionConfig::default().with_table_name("sessions");
    let auth_config = AuthConfig::<i64>::default();
    let session_store = SessionPgSessionStore::new(Some(pool.clone().into()), session_config)
        .await
        .unwrap();

    let conf = get_configuration(Some("Cargo.toml")).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app_state = AppState {
        leptos_options,
        routes: routes.clone(),
    };

    let app = Router::new()
        .route(
            "/api/:fn_name",
            get(server_function_handler).post(server_function_handler),
        )
        .leptos_routes_with_handler(routes, get(routes_handler))
        .fallback(fallback_handler)
        .layer(
            AuthSessionLayer::<User, i64, SessionPgPool, PgPool>::new(Some(pool.clone()))
                .with_config(auth_config),
        )
        .layer(SessionLayer::new(session_store))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap()
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
