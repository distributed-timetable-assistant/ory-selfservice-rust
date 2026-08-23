use crate::shared::state::AppState;
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Response};
use http::{HeaderMap, HeaderValue, StatusCode};
use leptos::prelude::RenderHtml;
use std::collections::HashMap;
use std::sync::Arc;
use http::header::CONTENT_TYPE;
use crate::core::ports::flow::FetchUiFlowHandler;
use crate::shared::error::AppError;
use crate::ui::pages::ErrorPage;

pub async fn fetch_ui(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Response, AppError> {
    let flow_handler = &state.flow_handlers.error;
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
            .map(|ui_node| leptos::view! { <ErrorPage title=ui_node.title description=ui_node.description /> }.to_html())
            .unwrap_or_default(),
    )
        .into_response())
}
