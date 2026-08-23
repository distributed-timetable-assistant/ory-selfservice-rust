use crate::core::domain::error::FlowResult;
use crate::core::domain::flow::{RegistrationUiNode, UiNodeResponse};
use crate::core::ports::flow::{FetchUiFlowHandler, ProxySubmitUiFlowHandler};
use crate::core::ports::services::identity::IdentityService;
use crate::core::ports::utils::url_rewrite::UrlRewriter;
use crate::core::usecases::flow_handlers::services::{
    handle_identity_fetch_flow, handle_identity_submit_flow,
};
use async_trait::async_trait;
use axum::body::Bytes;
use http::HeaderMap;
use std::collections::HashMap;
use std::sync::Arc;

pub struct RegistrationFlowHandler<I: IdentityService, R: UrlRewriter> {
    identity_service: Arc<I>,
    url_rewriter: Arc<R>,
}

impl<I: IdentityService, R: UrlRewriter> RegistrationFlowHandler<I, R> {
    pub fn new(identity_service: Arc<I>, url_rewriter: Arc<R>) -> Self {
        RegistrationFlowHandler {
            identity_service,
            url_rewriter,
        }
    }
}

#[async_trait]
impl<I: IdentityService, R: UrlRewriter> FetchUiFlowHandler<RegistrationUiNode>
    for RegistrationFlowHandler<I, R>
{
    async fn fetch_ui(
        &self,
        headers: &HeaderMap,
        params: &HashMap<String, String>,
    ) -> FlowResult<UiNodeResponse<RegistrationUiNode>> {
        let identity_service = self.identity_service.clone();
        let url_rewriter = self.url_rewriter.clone();
        let mut resp = handle_identity_fetch_flow::<RegistrationUiNode>(
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
impl<I: IdentityService, R: UrlRewriter> ProxySubmitUiFlowHandler<RegistrationUiNode>
    for RegistrationFlowHandler<I, R>
{
    async fn proxy_submit_ui(
        &self,
        headers: &HeaderMap,
        params: &HashMap<String, String>,
        body: Bytes,
    ) -> FlowResult<UiNodeResponse<RegistrationUiNode>> {
        let identity_service = self.identity_service.clone();
        let url_rewriter = self.url_rewriter.clone();
        let mut resp = handle_identity_submit_flow::<RegistrationUiNode>(
            headers,
            params,
            body,
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
