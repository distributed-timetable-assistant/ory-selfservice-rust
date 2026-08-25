use crate::core::domain::error::FlowResult;
use crate::core::domain::flow::{UiNodeResponse, VerificationUiNode};
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

pub struct VerificationFlowHandler<I: IdentityService, R: UrlRewriter> {
    identity_service: Arc<I>,
    url_rewriter: Arc<R>,
}

impl<I: IdentityService, R: UrlRewriter> VerificationFlowHandler<I, R> {
    pub fn new(identity_service: Arc<I>, url_rewriter: Arc<R>) -> Self {
        VerificationFlowHandler {
            identity_service,
            url_rewriter,
        }
    }
}

#[async_trait]
impl<I: IdentityService, R: UrlRewriter> FetchUiFlowHandler<VerificationUiNode>
    for VerificationFlowHandler<I, R>
{
    async fn fetch_ui(
        &self,
        headers: &HeaderMap,
        params: &HashMap<String, String>,
    ) -> FlowResult<UiNodeResponse<VerificationUiNode>> {
        let identity_service = self.identity_service.clone();
        let url_rewriter = self.url_rewriter.clone();
        let mut resp = handle_identity_fetch_flow::<VerificationUiNode>(
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
impl<I: IdentityService, R: UrlRewriter> ProxySubmitUiFlowHandler<VerificationUiNode>
    for VerificationFlowHandler<I, R>
{
    async fn proxy_submit_ui(
        &self,
        headers: &HeaderMap,
        params: &HashMap<String, String>,
        body: Bytes,
    ) -> FlowResult<UiNodeResponse<VerificationUiNode>> {
        let identity_service = self.identity_service.clone();
        let url_rewriter = self.url_rewriter.clone();
        let mut resp = handle_identity_submit_flow::<VerificationUiNode>(
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
