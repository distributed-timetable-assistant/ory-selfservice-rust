use crate::kratos::models::*;
use crate::shared::error::AppError;
use reqwest::header::{HeaderMap, COOKIE, USER_AGENT};

#[derive(Clone)]
pub struct KratosClient {
    pub client: reqwest::Client,
    public_url: String,
}

impl KratosClient {
    pub fn new(public_url: String, client: reqwest::Client) -> Self {
        Self { client, public_url }
    }

    pub(crate) fn copy_forward_headers(&self, incoming: &HeaderMap) -> HeaderMap {
        let mut outgoing = HeaderMap::new();

        // Forward Cookie header
        if let Some(cookie) = incoming.get(COOKIE) {
            outgoing.insert(COOKIE, cookie.clone());
        }

        // Forward User-Agent header
        if let Some(ua) = incoming.get(USER_AGENT) {
            outgoing.insert(USER_AGENT, ua.clone());
        }

        // Forward Accept and Accept-Language headers
        for header_name in &["accept", "accept-language"] {
            if let Some(val) = incoming.get(*header_name) {
                if let Ok(name) = reqwest::header::HeaderName::from_bytes(header_name.as_bytes()) {
                    outgoing.insert(name, val.clone());
                }
            }
        }

        // Forward IP forwarding headers
        for ip_header in &[
            "x-forwarded-for",
            "x-real-ip",
            "x-forwarded-proto",
            "x-forwarded-host",
        ] {
            if let Some(val) = incoming.get(*ip_header) {
                if let Ok(name) = reqwest::header::HeaderName::from_bytes(ip_header.as_bytes()) {
                    outgoing.insert(name, val.clone());
                }
            }
        }

        outgoing
    }

    fn validate_flow_id(&self, flow_id: &str) -> Result<(), AppError> {
        if !flow_id.chars().all(|c| c.is_ascii_hexdigit() || c == '-') {
            return Err(AppError::Kratos("Invalid flow ID format".to_string()));
        }
        Ok(())
    }

    pub async fn get_login_flow(
        &self,
        flow_id: &str,
        headers: &HeaderMap,
    ) -> Result<(LoginFlow, HeaderMap), AppError> {
        self.validate_flow_id(flow_id)?;
        let url = format!(
            "{}/self-service/login/flows?id={}",
            self.public_url, flow_id
        );
        let resp = self
            .client
            .get(&url)
            .headers(self.copy_forward_headers(headers))
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(AppError::Kratos(format!(
                "Failed to get login flow: {}",
                resp.status()
            )));
        }

        let mut response_headers = HeaderMap::new();
        for cookie in resp.headers().get_all(reqwest::header::SET_COOKIE) {
            response_headers.append(http::header::SET_COOKIE, cookie.clone());
        }

        let flow: LoginFlow = resp.json().await?;
        Ok((flow, response_headers))
    }

    pub async fn get_registration_flow(
        &self,
        flow_id: &str,
        headers: &HeaderMap,
    ) -> Result<(RegistrationFlow, HeaderMap), AppError> {
        self.validate_flow_id(flow_id)?;
        let url = format!(
            "{}/self-service/registration/flows?id={}",
            self.public_url, flow_id
        );
        let resp = self
            .client
            .get(&url)
            .headers(self.copy_forward_headers(headers))
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(AppError::Kratos(format!(
                "Failed to get registration flow: {}",
                resp.status()
            )));
        }

        let mut response_headers = HeaderMap::new();
        for cookie in resp.headers().get_all(reqwest::header::SET_COOKIE) {
            response_headers.append(http::header::SET_COOKIE, cookie.clone());
        }

        let flow: RegistrationFlow = resp.json().await?;
        Ok((flow, response_headers))
    }

    pub async fn get_recovery_flow(
        &self,
        flow_id: &str,
        headers: &HeaderMap,
    ) -> Result<(RecoveryFlow, HeaderMap), AppError> {
        self.validate_flow_id(flow_id)?;
        let url = format!(
            "{}/self-service/recovery/flows?id={}",
            self.public_url, flow_id
        );
        let resp = self
            .client
            .get(&url)
            .headers(self.copy_forward_headers(headers))
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(AppError::Kratos(format!(
                "Failed to get recovery flow: {}",
                resp.status()
            )));
        }

        let mut response_headers = HeaderMap::new();
        for cookie in resp.headers().get_all(reqwest::header::SET_COOKIE) {
            response_headers.append(http::header::SET_COOKIE, cookie.clone());
        }

        let flow: RecoveryFlow = resp.json().await?;
        Ok((flow, response_headers))
    }

    pub async fn get_verification_flow(
        &self,
        flow_id: &str,
        headers: &HeaderMap,
    ) -> Result<(VerificationFlow, HeaderMap), AppError> {
        self.validate_flow_id(flow_id)?;
        let url = format!(
            "{}/self-service/verification/flows?id={}",
            self.public_url, flow_id
        );
        let resp = self
            .client
            .get(&url)
            .headers(self.copy_forward_headers(headers))
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(AppError::Kratos(format!(
                "Failed to get verification flow: {}",
                resp.status()
            )));
        }

        let mut response_headers = HeaderMap::new();
        for cookie in resp.headers().get_all(reqwest::header::SET_COOKIE) {
            response_headers.append(http::header::SET_COOKIE, cookie.clone());
        }

        let flow: VerificationFlow = resp.json().await?;
        Ok((flow, response_headers))
    }

    pub async fn get_settings_flow(
        &self,
        flow_id: &str,
        headers: &HeaderMap,
    ) -> Result<(SettingsFlow, HeaderMap), AppError> {
        self.validate_flow_id(flow_id)?;
        let url = format!(
            "{}/self-service/settings/flows?id={}",
            self.public_url, flow_id
        );
        let resp = self
            .client
            .get(&url)
            .headers(self.copy_forward_headers(headers))
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(AppError::Kratos(format!(
                "Failed to get settings flow: {}",
                resp.status()
            )));
        }

        let mut response_headers = HeaderMap::new();
        for cookie in resp.headers().get_all(reqwest::header::SET_COOKIE) {
            response_headers.append(http::header::SET_COOKIE, cookie.clone());
        }

        let flow: SettingsFlow = resp.json().await?;
        Ok((flow, response_headers))
    }

    pub async fn check_session(&self, headers: &HeaderMap) -> Result<KratosSession, AppError> {
        let url = format!("{}/sessions/whoami", self.public_url);
        let resp = self
            .client
            .get(&url)
            .headers(self.copy_forward_headers(headers))
            .send()
            .await?;

        if resp.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(AppError::Unauthorized(
                "No active session found".to_string(),
            ));
        }

        if !resp.status().is_success() {
            return Err(AppError::Kratos(format!(
                "Failed checking session: {}",
                resp.status()
            )));
        }

        Ok(resp.json().await?)
    }

    /// Fetch a Kratos Self-Service error by its `id`.
    ///
    /// Calls `GET /self-service/errors?id={id}` (public, no session needed).
    /// Validates that the id contains only URL-safe characters to prevent
    /// open-redirect / path-traversal attacks.
    pub async fn get_error(&self, id: &str) -> Result<KratosErrorContainer, AppError> {
        // Reuse the same alphanumeric + hyphen allowlist used for flow IDs.
        if !id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            return Err(AppError::Kratos("Invalid error ID format".to_string()));
        }

        let url = format!("{}/self-service/errors?id={}", self.public_url, id);
        let resp = self.client.get(&url).send().await?;

        if !resp.status().is_success() {
            return Err(AppError::Kratos(format!(
                "Failed to fetch Kratos error: {}",
                resp.status()
            )));
        }

        Ok(resp.json().await?)
    }

    /// Initiate a Kratos browser-logout flow for the current session.
    ///
    /// Calls `GET /self-service/logout/browser`, forwarding the user's cookies
    /// so Kratos can identify the active session.
    ///
    /// `return_to` is optional.  When provided (e.g. a Hydra post-logout URI),
    /// it is appended as `?return_to=<url>` so that Kratos will redirect the
    /// browser to that URL **after** the session has been destroyed.
    pub async fn create_logout_flow(
        &self,
        headers: &HeaderMap,
        return_to: Option<&str>,
    ) -> Result<KratosLogoutFlow, AppError> {
        let mut url = format!("{}/self-service/logout/browser", self.public_url);
        if let Some(rt) = return_to {
            // URL-encode the return_to value so it is safe to embed in a query string.
            let encoded = serde_urlencoded::to_string(&[("return_to", rt)])
                .unwrap_or_else(|_| format!("return_to={}", rt));
            url = format!("{}?{}", url, encoded);
        }

        let resp = self
            .client
            .get(&url)
            .headers(self.copy_forward_headers(headers))
            .send()
            .await?;

        if resp.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(AppError::Unauthorized(
                "No active session — cannot create logout flow".to_string(),
            ));
        }

        if !resp.status().is_success() {
            return Err(AppError::Kratos(format!(
                "Failed to create logout flow: {}",
                resp.status()
            )));
        }

        Ok(resp.json().await?)
    }
}

