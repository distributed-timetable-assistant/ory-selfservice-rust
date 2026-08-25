use crate::core::domain::flow::ConsentSubmitNode;
use crate::core::ports::flow::{FetchUiFlowHandler, SubmitActionFlowHandler};
use crate::shared::error::AppError;
use crate::shared::state::AppState;
use crate::ui::pages::ConsentPage;
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Response};
use axum_extra::extract::Form;
use http::header::CONTENT_TYPE;
use http::{HeaderMap, HeaderValue};
use leptos::prelude::RenderHtml;
use std::collections::HashMap;
use std::sync::Arc;

pub(crate) async fn fetch_ui(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Response, AppError> {
    let flow_handler = &state.flow_handlers.consent;
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
            .map(|flow| leptos::view! { <ConsentPage req=flow /> }.to_html())
            .unwrap_or_default(),
    )
        .into_response())
}

pub(crate) async fn submit_action(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
    Form(submit): Form<ConsentSubmitNode>,
) -> Result<Response, AppError> {
    let flow_handler = &state.flow_handlers.consent;
    let resp = flow_handler
        .submit_action(&headers, &params, submit)
        .await?;
    Ok((resp.status_code, resp.headers).into_response())
}
