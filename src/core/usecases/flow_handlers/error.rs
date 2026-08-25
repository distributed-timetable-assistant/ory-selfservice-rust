use crate::core::domain::error::FlowResult;
use crate::core::domain::flow::{ErrorUiNode, UiNodeResponse};
use crate::core::ports::flow::FetchUiFlowHandler;
use crate::core::ports::services::identity::IdentityService;
use async_trait::async_trait;
use http::{HeaderMap, StatusCode};
use std::collections::HashMap;
use std::sync::Arc;

pub struct ErrorFlowHandler<U>
where
    U: IdentityService,
{
    identity_service: Arc<U>,
}

impl<U> ErrorFlowHandler<U>
where
    U: IdentityService,
{
    pub fn new(identity_service: Arc<U>) -> Self {
        ErrorFlowHandler { identity_service }
    }
}

#[async_trait]
impl<U> FetchUiFlowHandler<ErrorUiNode> for ErrorFlowHandler<U>
where
    U: IdentityService,
{
    async fn fetch_ui(
        &self,
        _headers: &HeaderMap,
        params: &HashMap<String, String>,
    ) -> FlowResult<UiNodeResponse<ErrorUiNode>> {
        // TODO: Issue #3
        let identity_service = self.identity_service.clone();
        // ── Priority 1: Kratos Self-Service error ID ──────────────────────────────
        if let Some(id) = params.get("id") {
            match identity_service.get_error(id).await {
                Ok(container) => {
                    let detail = container.error;
                    // Prefer `reason` as the headline, fall back to `status` or a generic title.
                    let title = detail
                        .status
                        .clone()
                        .unwrap_or_else(|| "An Error Occurred".to_string());
                    let description = detail.reason.or(detail.message).unwrap_or_else(|| {
                        "An unexpected error occurred. Please try again.".to_string()
                    });
                    let ui_node = ErrorUiNode { title, description };
                    return Ok(UiNodeResponse {
                        ui_node: Some(ui_node),
                        status_code: StatusCode::OK,
                        headers: Default::default(),
                    });
                }
                Err(_err) => {
                    let ui_node = ErrorUiNode {
                        title: "Error Details Unavailable".to_string(),
                        description: "The error details could not be retrieved. Please return to the login page and try again.".to_string(),
                    };
                    return Ok(UiNodeResponse {
                        ui_node: Some(ui_node),
                        status_code: StatusCode::INTERNAL_SERVER_ERROR,
                        headers: Default::default(),
                    });
                }
            }
        }

        // ── Priority 2: OAuth2 / OIDC error params ────────────────────────────────
        if let Some(error_code) = params.get("error") {
            let description = params
                .get("error_description")
                .cloned()
                .unwrap_or_else(|| format!("An OAuth2 error occurred: {}", error_code));
            // Convert snake_case code to a readable title (e.g. "access_denied" → "Access Denied")
            let title = error_code
                .replace('_', " ")
                .split_whitespace()
                .map(|word| {
                    let mut chars = word.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(first) => first.to_uppercase().to_string() + chars.as_str(),
                    }
                })
                .collect::<Vec<_>>()
                .join(" ");
            let ui_node = ErrorUiNode { title, description };
            return Ok(UiNodeResponse {
                ui_node: Some(ui_node),
                status_code: StatusCode::OK,
                headers: Default::default(),
            });
        }
        let ui_node = ErrorUiNode {
            title: "An Unknown Error Occurred".to_string(),
            description: "Something went wrong. Please return to the login page and try again."
                .to_string(),
        };
        Ok(UiNodeResponse {
            ui_node: Some(ui_node),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            headers: Default::default(),
        })
    }
}
