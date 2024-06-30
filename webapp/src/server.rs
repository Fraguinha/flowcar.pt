use crate::app::App;
use crate::models::user::User;
use crate::state::AppState;
use axum::response::Response as AxumResponse;
use axum::{
    body::Body,
    extract::State,
    http::{Request, Response, StatusCode},
    response::IntoResponse,
};
use axum_session_sqlx::SessionPgPool;
use leptos::*;
use tower::ServiceExt;
use tower_http::services::ServeDir;

use axum::extract::Path;
use axum_session_auth::AuthSession;
use leptos_axum::handle_server_fns_with_context;
use sqlx::PgPool;

pub async fn server_function_handler(
    State(_app_state): State<AppState>,
    auth_session: AuthSession<User, i64, SessionPgPool, PgPool>,
    _path: Path<String>,
    request: Request<Body>,
) -> impl IntoResponse {
    handle_server_fns_with_context(
        move || {
            provide_context(auth_session.clone());
        },
        request,
    )
    .await
}

pub async fn routes_handler(
    State(app_state): State<AppState>,
    auth_session: AuthSession<User, i64, SessionPgPool, PgPool>,
    req: Request<Body>,
) -> AxumResponse {
    let handler = leptos_axum::render_route_with_context(
        app_state.leptos_options.clone(),
        app_state.routes.clone(),
        move || {
            provide_context(auth_session.clone());
        },
        App,
    );
    handler(req).await.into_response()
}

pub async fn fallback_handler(
    State(options): State<LeptosOptions>,
    auth_session: AuthSession<User, i64, SessionPgPool, PgPool>,
    req: Request<Body>,
) -> AxumResponse {
    let root = options.site_root.clone();
    let (parts, body) = req.into_parts();

    let mut static_parts = parts.clone();
    static_parts.headers.clear();
    if let Some(encodings) = parts.headers.get("accept-encoding") {
        static_parts
            .headers
            .insert("accept-encoding", encodings.clone());
    }

    let res = get_static_file(Request::from_parts(static_parts, Body::empty()), &root)
        .await
        .unwrap();

    if res.status() == StatusCode::OK {
        res.into_response()
    } else {
        let handler = leptos_axum::render_app_to_stream_with_context(
            options.to_owned(),
            move || {
                provide_context(auth_session.clone());
            },
            App,
        );
        handler(Request::from_parts(parts, body))
            .await
            .into_response()
    }
}

#[allow(unreachable_patterns)]
async fn get_static_file(
    request: Request<Body>,
    root: &str,
) -> Result<Response<Body>, (StatusCode, String)> {
    match ServeDir::new(root)
        .precompressed_gzip()
        .precompressed_br()
        .oneshot(request)
        .await
    {
        Ok(res) => Ok(res.into_response()),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error serving files: {err}"),
        )),
    }
}
