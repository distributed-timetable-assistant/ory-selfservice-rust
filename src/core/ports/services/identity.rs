use crate::core::domain::error::IdentityResult;
use crate::core::domain::flow::KratosLogoutFlow;
use crate::core::domain::flow::{FlowType, KratosErrorContainer, KratosSession};
use crate::core::ports::flow::UiNodeVariant;
use async_trait::async_trait;
use axum::body::Bytes;
use http::HeaderMap;
use reqwest::Response;
use std::collections::HashMap;

#[async_trait]
pub trait IdentityService: Send + Sync {
    async fn fetch_flow<U: UiNodeVariant>(
        &self,
        headers: &HeaderMap,
        params: &HashMap<String, String>,
    ) -> IdentityResult<Response>;
    async fn proxy_submit_flow(
        &self,
        flow_type: &FlowType,
        headers: &HeaderMap,
        params: &HashMap<String, String>,
        body: Bytes,
    ) -> IdentityResult<Response>;
    async fn check_session(&self, headers: &HeaderMap) -> IdentityResult<KratosSession>;
    async fn get_error(&self, id: &str) -> IdentityResult<KratosErrorContainer>;
    async fn create_logout_flow(
        &self,
        headers: &HeaderMap,
        return_to: Option<&str>,
    ) -> IdentityResult<KratosLogoutFlow>;
}
