use crate::core::domain::error::{OidcError, OidcResult};
use crate::core::domain::flow::ConsentUiNode;
use crate::core::domain::oidc_domains::{
    AcceptConsentRequest, AcceptLoginRequest, CompletedRequest, LoginRequest, LogoutRequest,
    RejectRequest,
};
use crate::core::ports::services::oidc::OidcService;
use async_trait::async_trait;
use reqwest_middleware::ClientWithMiddleware;

#[derive(Clone)]
pub struct HydraService {
    pub client: ClientWithMiddleware,
    admin_url: String,
}

impl HydraService {
    pub fn new(admin_url: String, client: ClientWithMiddleware) -> Self {
        Self { client, admin_url }
    }

    // TODO: Issue #2
    pub async fn reject_login_request(
        &self,
        challenge: &str,
        body: RejectRequest,
    ) -> OidcResult<CompletedRequest> {
        let url = format!(
            "{}/oauth2/auth/requests/login/reject?login_challenge={}",
            self.admin_url, challenge
        );
        let resp = self.client.put(&url).json(&body).send().await?;

        if !resp.status().is_success() {
            return Err(OidcError::RetrieveRequest(format!(
                "Failed to reject login request: {}",
                resp.status()
            )));
        }

        Ok(resp.json().await?)
    }

    /// Fetch the Hydra logout request for an RP-Initiated Logout challenge.
    ///
    /// Calls `GET /oauth2/auth/requests/logout?logout_challenge={challenge}`.
    pub async fn get_logout_request(&self, challenge: &str) -> OidcResult<LogoutRequest> {
        let url = format!(
            "{}/oauth2/auth/requests/logout?logout_challenge={}",
            self.admin_url, challenge
        );
        let resp = self.client.get(&url).send().await?;

        if !resp.status().is_success() {
            return Err(OidcError::RetrieveRequest(format!(
                "Failed to retrieve logout request: {}",
                resp.status()
            )));
        }

        Ok(resp.json().await?)
    }
}

#[async_trait]
impl OidcService for HydraService {
    async fn get_login_request(&self, challenge: &str) -> OidcResult<LoginRequest> {
        let url = format!(
            "{}/oauth2/auth/requests/login?login_challenge={}",
            self.admin_url, challenge
        );
        let resp = self.client.get(&url).send().await?;
        if !resp.status().is_success() {
            return Err(OidcError::RetrieveRequest(format!(
                "Failed to retrieve login request. status_code=({})",
                resp.status()
            )));
        }
        Ok(resp.json().await?)
    }

    async fn accept_login_request(
        &self,
        challenge: &str,
        body: AcceptLoginRequest,
    ) -> OidcResult<CompletedRequest> {
        let url = format!(
            "{}/oauth2/auth/requests/login/accept?login_challenge={}",
            self.admin_url, challenge
        );
        let resp = self.client.put(&url).json(&body).send().await?;
        if !resp.status().is_success() {
            return Err(OidcError::AcceptRequest(format!(
                "Failed to accept login request. status_code=({})",
                resp.status()
            )));
        }
        Ok(resp.json().await?)
    }

    async fn get_consent_request(&self, challenge: &str) -> OidcResult<ConsentUiNode> {
        let url = format!(
            "{}/oauth2/auth/requests/consent?consent_challenge={}",
            self.admin_url, challenge
        );
        let resp = self.client.get(&url).send().await?;
        if !resp.status().is_success() {
            return Err(OidcError::RetrieveRequest(format!(
                "Failed to retrieve consent request. status_code=({})",
                resp.status()
            )));
        }
        Ok(resp.json().await?)
    }

    async fn accept_consent_request(
        &self,
        challenge: &str,
        body: AcceptConsentRequest,
    ) -> OidcResult<CompletedRequest> {
        let url = format!(
            "{}/oauth2/auth/requests/consent/accept?consent_challenge={}",
            self.admin_url, challenge
        );
        let resp = self.client.put(&url).json(&body).send().await?;
        if !resp.status().is_success() {
            return Err(OidcError::AcceptRequest(format!(
                "Failed to accept consent request. status_code=({})",
                resp.status()
            )));
        }
        Ok(resp.json().await?)
    }

    async fn reject_consent_request(
        &self,
        challenge: &str,
        body: RejectRequest,
    ) -> OidcResult<CompletedRequest> {
        let url = format!(
            "{}/oauth2/auth/requests/consent/reject?consent_challenge={}",
            self.admin_url, challenge
        );
        let resp = self.client.put(&url).json(&body).send().await?;
        if !resp.status().is_success() {
            return Err(OidcError::AcceptRequest(format!(
                "Failed to reject consent request. status_code=({})",
                resp.status()
            )));
        }
        Ok(resp.json().await?)
    }

    async fn accept_logout_request(&self, challenge: &str) -> OidcResult<CompletedRequest> {
        let url = format!(
            "{}/oauth2/auth/requests/logout/accept?logout_challenge={}",
            self.admin_url, challenge
        );

        let resp = self
            .client
            .put(&url)
            .json(&serde_json::json!({}))
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(OidcError::AcceptRequest(format!(
                "Failed to accept logout request. status_code=({})",
                resp.status()
            )));
        }

        Ok(resp.json().await?)
    }
}
