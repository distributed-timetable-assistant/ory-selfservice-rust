use crate::core::ports::flow::FetchActionFlowHandler;
use crate::shared::error::AppError;
use crate::shared::state::AppState;
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Response};
use http::HeaderMap;
use std::collections::HashMap;
use std::sync::Arc;

pub(crate) async fn get_logout(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Response, AppError> {
    let flow_handler = &state.flow_handlers.logout;
    let resp = flow_handler.fetch_action(&headers, &params).await?;
    Ok((resp.status_code, resp.headers).into_response())
}
