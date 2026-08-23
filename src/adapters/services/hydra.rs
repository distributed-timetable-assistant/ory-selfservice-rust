use crate::core::domain::error::{OidcError, OidcResult};
use crate::core::domain::flow::ConsentUiNode;
use crate::core::domain::oidc_domains::{
    AcceptConsentRequest, AcceptLoginRequest, CompletedRequest, LoginRequest, LogoutRequest,
    RejectRequest,
};
use crate::core::ports::services::oidc::OidcService;
use crate::shared::error::AppError;
use async_trait::async_trait;

#[derive(Clone)]
pub struct HydraService {
    pub client: reqwest::Client,
    admin_url: String,
}

impl HydraService {
    pub fn new(admin_url: String) -> Self {
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("Failed to build reqwest client for Hydra");

        Self { client, admin_url }
    }

    // TODO: Use if it's needed
    pub async fn reject_login_request(
        &self,
        challenge: &str,
        body: RejectRequest,
    ) -> Result<CompletedRequest, AppError> {
        let url = format!(
            "{}/oauth2/auth/requests/login/reject?login_challenge={}",
            self.admin_url, challenge
        );
        let resp = self.client.put(&url).json(&body).send().await?;

        if !resp.status().is_success() {
            return Err(AppError::Hydra(format!(
                "Failed to reject login request: {}",
                resp.status()
            )));
        }

        Ok(resp.json().await?)
    }

    // TODO: Use if it's needed
    /// Fetch the Hydra logout request for an RP-Initiated Logout challenge.
    ///
    /// Calls `GET /oauth2/auth/requests/logout?logout_challenge={challenge}`.
    pub async fn get_logout_request(&self, challenge: &str) -> Result<LogoutRequest, AppError> {
        let url = format!(
            "{}/oauth2/auth/requests/logout?logout_challenge={}",
            self.admin_url, challenge
        );
        let resp = self.client.get(&url).send().await?;

        if !resp.status().is_success() {
            return Err(AppError::Hydra(format!(
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

    async fn accept_logout_request(
        &self,
        challenge: &str,
    ) -> OidcResult<CompletedRequest> {
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
