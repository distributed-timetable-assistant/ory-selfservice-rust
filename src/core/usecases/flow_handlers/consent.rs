use crate::core::domain::error::{FlowError, FlowResult};
use crate::core::domain::flow::ConsentSubmitNode;
use crate::core::domain::flow::{ActionResponse, ConsentUiNode, UiNodeResponse};
use crate::core::domain::oidc_domains::{AcceptConsentRequest, RejectRequest};
use crate::core::ports::flow::{FetchUiFlowHandler, SubmitActionFlowHandler};
use crate::core::ports::services::identity::IdentityService;
use crate::core::ports::services::oidc::OidcService;
use async_trait::async_trait;
use http::header::LOCATION;
use http::{HeaderMap, HeaderValue, StatusCode};
use std::collections::HashMap;
use std::sync::Arc;

pub struct ConsentFlowHandler<I, O>
where
    I: IdentityService,
    O: OidcService,
{
    identity_service: Arc<I>,
    oidc_service: Arc<O>,
}

impl<I, O> ConsentFlowHandler<I, O>
where
    I: IdentityService,
    O: OidcService,
{
    pub fn new(identity_service: Arc<I>, oidc_service: Arc<O>) -> Self {
        ConsentFlowHandler {
            identity_service,
            oidc_service,
        }
    }
}

#[async_trait]
impl<I, O> FetchUiFlowHandler<ConsentUiNode> for ConsentFlowHandler<I, O>
where
    I: IdentityService,
    O: OidcService,
{
    async fn fetch_ui(
        &self,
        headers: &HeaderMap,
        params: &HashMap<String, String>,
    ) -> FlowResult<UiNodeResponse<ConsentUiNode>> {
        let challenge = params.get("consent_challenge").ok_or_else(|| {
            FlowError::ConsentChallenge(format!(
                "Consent challenge parameter not found. params=({:?})",
                params
            ))
        })?;
        let ui_node = self.oidc_service.get_consent_request(challenge).await?;

        if ui_node.skip.unwrap_or(false) {
            let kratos_session = self.identity_service.check_session(&headers).await?;

            let email = kratos_session
                .identity
                .traits
                .get("email")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();

            let subject = kratos_session.identity.id;

            let session = Some(crate::core::domain::oidc_domains::ConsentRequestSession {
                id_token: Some(serde_json::json!({
                    "sub": subject,
                    "email": email,
                })),
                access_token: None,
            });

            let accept_res = self
                .oidc_service
                .accept_consent_request(
                    challenge,
                    AcceptConsentRequest {
                        grant_scope: ui_node.requested_scope,
                        grant_access_token_audience: ui_node.requested_access_token_audience,
                        remember: Some(true),
                        remember_for: Some(3600),
                        session,
                    },
                )
                .await?;

            let headers =
                HeaderMap::from_iter([(LOCATION, HeaderValue::from_str(&accept_res.redirect_to)?)]);
            return Ok(UiNodeResponse {
                ui_node: None,
                status_code: StatusCode::SEE_OTHER,
                headers,
            });
        }

        Ok(UiNodeResponse {
            ui_node: Some(ui_node),
            status_code: StatusCode::OK,
            headers: Default::default(),
        })
    }
}

#[async_trait]
impl<I, O> SubmitActionFlowHandler<ConsentSubmitNode> for ConsentFlowHandler<I, O>
where
    I: IdentityService,
    O: OidcService,
{
    async fn submit_action(
        &self,
        headers: &HeaderMap,
        _params: &HashMap<String, String>,
        submit: ConsentSubmitNode,
    ) -> FlowResult<ActionResponse> {
        if submit.submit == "accept" {
            let grant_scope = submit.grant_scope.unwrap_or_default();
            let kratos_session = self.identity_service.check_session(&headers).await?;
            let email = kratos_session
                .identity
                .traits
                .get("email")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();

            let subject = kratos_session.identity.id;

            let session = Some(crate::core::domain::oidc_domains::ConsentRequestSession {
                id_token: Some(serde_json::json!({
                    "sub": subject,
                    "email": email,
                })),
                access_token: None,
            });

            let accept_res = self
                .oidc_service
                .accept_consent_request(
                    &submit.consent_challenge,
                    AcceptConsentRequest {
                        grant_scope,
                        grant_access_token_audience: None,
                        remember: Some(true),
                        remember_for: Some(3600),
                        session,
                    },
                )
                .await?;
            let headers =
                HeaderMap::from_iter([(LOCATION, HeaderValue::from_str(&accept_res.redirect_to)?)]);
            Ok(ActionResponse {
                status_code: StatusCode::SEE_OTHER,
                headers,
            })
        } else {
            // Handle consent rejection
            let reject_res = self
                .oidc_service
                .reject_consent_request(
                    &submit.consent_challenge,
                    RejectRequest {
                        error: "consent_denied".to_string(),
                        error_description: Some("The user denied the consent request.".to_string()),
                        error_uri: None,
                        status_code: Some(StatusCode::FORBIDDEN.as_u16() as i64),
                    },
                )
                .await?;
            let headers =
                HeaderMap::from_iter([(LOCATION, HeaderValue::from_str(&reject_res.redirect_to)?)]);
            Ok(ActionResponse {
                status_code: StatusCode::SEE_OTHER,
                headers,
            })
        }
    }
}
