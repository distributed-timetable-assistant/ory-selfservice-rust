use crate::core::domain::error::{FlowError, FlowResult};
use crate::core::domain::flow::{ActionResponse, LoginUiNode, UiNodeResponse};
use crate::core::domain::oidc_domains::AcceptLoginRequest;
use crate::core::ports::flow::{FetchActionFlowHandler, FetchUiFlowHandler, ProxySubmitUiFlowHandler};
use crate::core::ports::services::identity::IdentityService;
use crate::core::ports::services::oidc::OidcService;
use crate::core::ports::utils::url_rewrite::UrlRewriter;
use crate::core::usecases::flow_handlers::services::{
    handle_identity_fetch_flow, handle_identity_submit_flow,
};
use async_trait::async_trait;
use axum::body::Bytes;
use http::header::LOCATION;
use http::{HeaderMap, HeaderValue, StatusCode};
use std::collections::HashMap;
use std::sync::Arc;

pub struct LoginFlowHandler<I, O, R>
where
    I: IdentityService,
    O: OidcService,
    R: UrlRewriter,
{
    identity_service: Arc<I>,
    oidc_service: Arc<O>,
    url_rewriter: Arc<R>,
}

impl<I, O, R> LoginFlowHandler<I, O, R>
where
    I: IdentityService,
    O: OidcService,
    R: UrlRewriter,
{
    pub fn new(identity_service: Arc<I>, oidc_service: Arc<O>, url_rewriter: Arc<R>) -> Self {
        LoginFlowHandler {
            identity_service,
            oidc_service,
            url_rewriter,
        }
    }
}

#[async_trait]
impl<I, O, R> FetchUiFlowHandler<LoginUiNode> for LoginFlowHandler<I, O, R>
where
    I: IdentityService,
    O: OidcService,
    R: UrlRewriter,
{
    async fn fetch_ui(
        &self,
        headers: &HeaderMap,
        params: &HashMap<String, String>,
    ) -> FlowResult<UiNodeResponse<LoginUiNode>> {
        let identity_service = self.identity_service.clone();
        let url_rewriter = self.url_rewriter.clone();
        let mut resp = handle_identity_fetch_flow::<LoginUiNode>(
            headers,
            params,
            identity_service,
            url_rewriter,
        )
        .await?;
        if let Some(flow) = &mut resp.ui_node {
            flow.ui.action = self.url_rewriter.rewrite(&flow.ui.action);
        }
        Ok(resp)
    }
}

#[async_trait]
impl<I, O, R> ProxySubmitUiFlowHandler<LoginUiNode> for LoginFlowHandler<I, O, R>
where
    I: IdentityService,
    O: OidcService,
    R: UrlRewriter,
{
    async fn proxy_submit_ui(
        &self,
        headers: &HeaderMap,
        params: &HashMap<String, String>,
        body: Bytes,
    ) -> FlowResult<UiNodeResponse<LoginUiNode>> {
        let identity_service = self.identity_service.clone();
        let url_rewriter = self.url_rewriter.clone();
        let mut resp = handle_identity_submit_flow::<LoginUiNode>(
            headers,
            params,
            body,
            identity_service,
            url_rewriter,
        )
        .await?;
        if let Some(ui_node) = &mut resp.ui_node {
            ui_node.ui.action = self.url_rewriter.rewrite(&ui_node.ui.action);
        }
        Ok(resp)
    }
}

#[async_trait]
impl<I, O, R> FetchActionFlowHandler for LoginFlowHandler<I, O, R>
where
    I: IdentityService,
    O: OidcService,
    R: UrlRewriter,
{
    async fn fetch_action(
        &self,
        headers: &HeaderMap,
        params: &HashMap<String, String>,
    ) -> FlowResult<ActionResponse> {
        let challenge = params.get("login_challenge").ok_or_else(|| {
            FlowError::LoginChallenge(format!(
                "Login challenge parameter not found. params=({:?})",
                params
            ))
        })?;
        let login_req = self.oidc_service.get_login_request(challenge).await?;

        if login_req.skip {
            let subject = login_req.subject.clone();
            let accept_res = self
                .oidc_service
                .accept_login_request(
                    challenge,
                    AcceptLoginRequest {
                        subject: subject.ok_or_else(|| {
                            FlowError::Subject(
                                format!("Missing subject in login request despite skip=true. login_req=({:?})",
                                        login_req))})?,
                        remember: Some(true),
                        remember_for: Some(3600),
                        acr: None,
                        context: None,
                    },
                )
                .await?;

            let headers =
                HeaderMap::from_iter([(LOCATION, HeaderValue::from_str(&accept_res.redirect_to)?)]);
            return Ok(ActionResponse {
                status_code: StatusCode::SEE_OTHER,
                headers,
            });
        }

        match self.identity_service.check_session(&headers).await {
            Ok(session) => {
                let accept_res = self
                    .oidc_service
                    .accept_login_request(
                        challenge,
                        AcceptLoginRequest {
                            subject: session.identity.id,
                            remember: Some(true),
                            remember_for: Some(3600),
                            acr: None,
                            context: None,
                        },
                    )
                    .await?;
                let headers = HeaderMap::from_iter([(
                    LOCATION,
                    HeaderValue::from_str(&accept_res.redirect_to)?,
                )]);
                Ok(ActionResponse {
                    status_code: StatusCode::SEE_OTHER,
                    headers,
                })
            }
            Err(_) => {
                let encoded_challenge =
                    serde_urlencoded::to_string(&[("login_challenge", challenge)])
                        .unwrap_or_default();
                let redirect_url = format!("/login?{}", encoded_challenge);
                let headers =
                    HeaderMap::from_iter([(LOCATION, HeaderValue::from_str(&redirect_url)?)]);
                Ok(ActionResponse {
                    status_code: StatusCode::SEE_OTHER,
                    headers,
                })
            }
        }
    }
}
