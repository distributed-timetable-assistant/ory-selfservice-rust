use crate::core::ports::flow::{FetchUiFlowHandler, ProxySubmitUiFlowHandler};
use crate::shared::error::AppError;
use crate::shared::state::AppState;
use crate::ui::pages::VerificationPage;
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Response};
use http::header::CONTENT_TYPE;
use http::{HeaderMap, HeaderValue};
use leptos::prelude::RenderHtml;
use std::collections::HashMap;
use std::sync::Arc;

pub async fn fetch_ui(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Response, AppError> {
    let flow_handler = &state.flow_handlers.verification;
    let mut resp = flow_handler.fetch_ui(&headers, &params).await?;
    if resp.ui_node.is_some() {
        resp.headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("text/html; charset=utf-8"),
        );
    }
    Ok((
        resp.status_code,
        resp.headers,
        resp.ui_node
            .map(|ui_node| leptos::view! { <VerificationPage flow=ui_node /> }.to_html())
            .unwrap_or_default(),
    )
        .into_response())
}

pub async fn proxy_submit_ui(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
    body: axum::body::Bytes,
) -> Result<Response, AppError> {
    let flow_handler = &state.flow_handlers.verification;
    let mut resp = flow_handler.proxy_submit_ui(&headers, &params, body).await?;
    if resp.ui_node.is_some() {
        resp.headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("text/html; charset=utf-8"),
        );
    }
    Ok((
        resp.status_code,
        resp.headers,
        resp.ui_node
            .map(|ui_node| leptos::view! { <VerificationPage flow=ui_node /> }.to_html())
            .unwrap_or_default(),
    )
        .into_response())
}
