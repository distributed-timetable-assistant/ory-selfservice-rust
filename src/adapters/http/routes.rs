use crate::adapters::http::endpoints::health::health_check;
use crate::adapters::http::endpoints::{consent, error, login, logout, recovery, registration, settings, verification};
use crate::shared::state::AppState;
use axum::response::Redirect;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(|| async { Redirect::temporary("/settings") }))
        .route("/login", get(login::fetch_ui).post(login::proxy_submit_ui))
        .route(
            "/registration",
            get(registration::fetch_ui).post(registration::proxy_submit_ui),
        )
        .route("/recovery", get(recovery::fetch_ui).post(recovery::proxy_submit_ui))
        .route(
            "/verification",
            get(verification::fetch_ui).post(verification::proxy_submit_ui),
        )
        .route("/settings", get(settings::fetch_ui).post(settings::proxy_submit_ui))
        .route("/oauth2/login", get(login::fetch_action))
        .route("/oauth2/consent", get(consent::fetch_ui).post(consent::submit_action))
        .route("/error", get(error::fetch_ui))
        .route("/logout", get(logout::get_logout))
        // DevOps handlers
        .route("/healthz", get(health_check))
}
