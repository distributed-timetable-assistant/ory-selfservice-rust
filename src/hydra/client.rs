use crate::hydra::models::*;
use crate::shared::error::AppError;

#[derive(Clone)]
pub struct HydraClient {
    pub client: reqwest::Client,
    admin_url: String,
}

impl HydraClient {
    pub fn new(admin_url: String) -> Self {
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("Failed to build reqwest client for Hydra");

        Self { client, admin_url }
    }

    pub async fn get_login_request(&self, challenge: &str) -> Result<LoginRequest, AppError> {
        let url = format!(
            "{}/oauth2/auth/requests/login?login_challenge={}",
            self.admin_url, challenge
        );
        let resp = self.client.get(&url).send().await?;

        if !resp.status().is_success() {
            return Err(AppError::Hydra(format!(
                "Failed to retrieve login request: {}",
                resp.status()
            )));
        }

        Ok(resp.json().await?)
    }

    pub async fn accept_login_request(
        &self,
        challenge: &str,
        body: AcceptLoginRequest,
    ) -> Result<CompletedRequest, AppError> {
        let url = format!(
            "{}/oauth2/auth/requests/login/accept?login_challenge={}",
            self.admin_url, challenge
        );
        let resp = self.client.put(&url).json(&body).send().await?;

        if !resp.status().is_success() {
            return Err(AppError::Hydra(format!(
                "Failed to accept login request: {}",
                resp.status()
            )));
        }

        Ok(resp.json().await?)
    }

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

    pub async fn get_consent_request(&self, challenge: &str) -> Result<ConsentRequest, AppError> {
        let url = format!(
            "{}/oauth2/auth/requests/consent?consent_challenge={}",
            self.admin_url, challenge
        );
        let resp = self.client.get(&url).send().await?;

        if !resp.status().is_success() {
            return Err(AppError::Hydra(format!(
                "Failed to retrieve consent request: {}",
                resp.status()
            )));
        }

        Ok(resp.json().await?)
    }

    pub async fn accept_consent_request(
        &self,
        challenge: &str,
        body: AcceptConsentRequest,
    ) -> Result<CompletedRequest, AppError> {
        let url = format!(
            "{}/oauth2/auth/requests/consent/accept?consent_challenge={}",
            self.admin_url, challenge
        );
        let resp = self.client.put(&url).json(&body).send().await?;

        if !resp.status().is_success() {
            return Err(AppError::Hydra(format!(
                "Failed to accept consent request: {}",
                resp.status()
            )));
        }

        Ok(resp.json().await?)
    }

    pub async fn reject_consent_request(
        &self,
        challenge: &str,
        body: RejectRequest,
    ) -> Result<CompletedRequest, AppError> {
        let url = format!(
            "{}/oauth2/auth/requests/consent/reject?consent_challenge={}",
            self.admin_url, challenge
        );
        let resp = self.client.put(&url).json(&body).send().await?;

        if !resp.status().is_success() {
            return Err(AppError::Hydra(format!(
                "Failed to reject consent request: {}",
                resp.status()
            )));
        }

        Ok(resp.json().await?)
    }

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

    /// Accept a Hydra RP-Initiated Logout challenge.
    ///
    /// Calls `PUT /oauth2/auth/requests/logout/accept?logout_challenge={challenge}`
    /// with an empty JSON body.  Hydra returns a `CompletedRequest` whose
    /// `redirect_to` is the URL the browser must visit after the Kratos session
    /// has been destroyed (typically the RP's post-logout redirect URI).
    pub async fn accept_logout_request(&self, challenge: &str) -> Result<CompletedRequest, AppError> {
        let url = format!(
            "{}/oauth2/auth/requests/logout/accept?logout_challenge={}",
            self.admin_url, challenge
        );
        // Hydra accepts an empty JSON body for the logout accept endpoint.
        let resp = self
            .client
            .put(&url)
            .json(&serde_json::json!({}))
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(AppError::Hydra(format!(
                "Failed to accept logout request: {}",
                resp.status()
            )));
        }

        Ok(resp.json().await?)
    }
}

