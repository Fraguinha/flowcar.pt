use axum::extract::FromRef;
use leptos::LeptosOptions;
use leptos_router::RouteListing;

#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub routes: Vec<RouteListing>,
}
