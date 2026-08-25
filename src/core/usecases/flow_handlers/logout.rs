use crate::core::domain::error::{FlowError, FlowResult, IdentityError};
use crate::core::domain::flow::ActionResponse;
use crate::core::ports::flow::FetchActionFlowHandler;
use crate::core::ports::services::identity::IdentityService;
use crate::core::ports::services::oidc::OidcService;
use async_trait::async_trait;
use http::header::LOCATION;
use http::{HeaderMap, HeaderValue, StatusCode};
use std::collections::HashMap;
use std::sync::Arc;

pub struct LogoutFlowHandler<I, O>
where
    I: IdentityService,
    O: OidcService,
{
    identity_service: Arc<I>,
    oidc_service: Arc<O>,
}

impl<I, O> LogoutFlowHandler<I, O>
where
    I: IdentityService,
    O: OidcService,
{
    pub fn new(identity_service: Arc<I>, oidc_service: Arc<O>) -> Self {
        LogoutFlowHandler {
            identity_service,
            oidc_service,
        }
    }
}

#[async_trait]
impl<I, O> FetchActionFlowHandler for LogoutFlowHandler<I, O>
where
    I: IdentityService,
    O: OidcService,
{
    async fn fetch_action(
        &self,
        headers: &HeaderMap,
        params: &HashMap<String, String>,
    ) -> FlowResult<ActionResponse> {
        // ── Hydra RP-Initiated Logout ─────────────────────────────────────────────
        if let Some(challenge) = params.get("logout_challenge") {
            // Step 1: Accept the logout challenge.  Hydra returns the OIDC client's
            // post-logout redirect URI in `redirect_to`.
            let completed = self.oidc_service.accept_logout_request(challenge).await?;
            let post_logout_redirect = completed.redirect_to;

            // Step 2: Ask Kratos to create the logout flow, passing the Hydra
            // post-logout URL as `return_to`.  Kratos will embed it in `logout_url`
            // so the browser ends up at the OIDC client's URI after session destruction.
            let flow = self
                .identity_service
                .create_logout_flow(&headers, Some(&post_logout_redirect))
                .await?;

            let headers =
                HeaderMap::from_iter([(LOCATION, HeaderValue::from_str(&flow.logout_url)?)]);
            return Ok(ActionResponse {
                status_code: StatusCode::SEE_OTHER,
                headers,
            });
        }

        // Direct Kratos Logout
        match self
            .identity_service
            .create_logout_flow(&headers, None)
            .await
        {
            Ok(flow) => {
                tracing::info!("Redirecting to Kratos logout URL: {}", flow.logout_url);
                let headers =
                    HeaderMap::from_iter([(LOCATION, HeaderValue::from_str(&flow.logout_url)?)]);
                Ok(ActionResponse {
                    status_code: StatusCode::SEE_OTHER,
                    headers,
                })
            }
            Err(IdentityError::Unauthorized(_)) => {
                let headers = HeaderMap::from_iter([(LOCATION, HeaderValue::from_str("/login")?)]);
                Ok(ActionResponse {
                    status_code: StatusCode::SEE_OTHER,
                    headers,
                })
            }
            Err(err) => Err(FlowError::from(err)),
        }
    }
}
