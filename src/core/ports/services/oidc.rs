use crate::core::domain::error::OidcResult;
use crate::core::domain::flow::ConsentUiNode;
use crate::core::domain::oidc_domains::{AcceptConsentRequest, AcceptLoginRequest, CompletedRequest, LoginRequest, RejectRequest};
use async_trait::async_trait;

#[async_trait]
pub trait OidcService: Send + Sync {
    async fn get_login_request(&self, challenge: &str) -> OidcResult<LoginRequest>;
    async fn accept_login_request(
        &self,
        challenge: &str,
        body: AcceptLoginRequest,
    ) -> OidcResult<CompletedRequest>;
    async fn get_consent_request(&self, challenge: &str) -> OidcResult<ConsentUiNode>;
    async fn accept_consent_request(
        &self,
        challenge: &str,
        body: AcceptConsentRequest,
    ) -> OidcResult<CompletedRequest>;
    async fn reject_consent_request(
        &self,
        challenge: &str,
        body: RejectRequest,
    ) -> OidcResult<CompletedRequest>;
    async fn accept_logout_request(
        &self,
        challenge: &str,
    ) -> OidcResult<CompletedRequest>;
}