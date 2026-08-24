use crate::core::domain::error::{IdentityError, IdentityResult};
use crate::core::domain::flow::KratosLogoutFlow;
use crate::core::domain::flow::{FlowType, KratosErrorContainer, KratosSession};
use crate::core::ports::flow::UiNodeVariant;
use crate::core::ports::services::identity::IdentityService;
use async_trait::async_trait;
use axum::body::Bytes;
use http::header::{ACCEPT, ACCEPT_LANGUAGE, CONTENT_TYPE, COOKIE, USER_AGENT};
use http::{HeaderMap, HeaderName, StatusCode};
use reqwest::Response;
use reqwest_middleware::ClientWithMiddleware;
use std::collections::HashMap;

#[derive(Clone)]
pub struct KratosService {
    pub client: ClientWithMiddleware,
    public_url: String,
}

impl KratosService {
    pub fn new(public_url: String, client: ClientWithMiddleware) -> Self {
        Self { client, public_url }
    }

    fn target_to_identity(
        &self,
        flow_type: &FlowType,
        target_path: Option<&str>,
        params: &HashMap<String, String>,
    ) -> String {
        let base = format!(
            "{}/self-service/{}",
            self.public_url,
            flow_type.to_string().to_lowercase()
        );
        let path = target_path.map(|p| format!("/{p}")).unwrap_or_default();
        if !params.is_empty() {
            return format!(
                "{}{}?{}",
                base,
                path,
                serde_urlencoded::to_string(params).unwrap_or_default()
            );
        }
        format!("{}{}", base, path)
    }

    fn validate_flow_id(&self, flow_id: &str) -> IdentityResult<()> {
        if !flow_id.chars().all(|c| c.is_ascii_hexdigit() || c == '-') {
            return Err(IdentityError::IncorrectFlowIdFormat(flow_id.into()));
        }
        Ok(())
    }

    fn validate_error_id(&self, error_id: &str) -> IdentityResult<()> {
        if !error_id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-')
        {
            return Err(IdentityError::IncorrectErrorIdFormat(error_id.to_owned()));
        }
        Ok(())
    }

    fn copy_forward_headers(
        &self,
        incoming: &HeaderMap,
        extra_keys: impl IntoIterator<Item = HeaderName>,
    ) -> HeaderMap {
        let common_keys = vec![
            COOKIE,
            USER_AGENT,
            ACCEPT,
            ACCEPT_LANGUAGE,
            HeaderName::from_static("x-forwarded-for"),
            HeaderName::from_static("x-real-ip"),
            HeaderName::from_static("x-forwarded-proto"),
            HeaderName::from_static("x-forwarded-host"),
        ];

        common_keys
            .into_iter()
            .chain(extra_keys)
            .flat_map(|key| {
                incoming
                    .get_all(&key)
                    .into_iter()
                    .map(move |value| (key.clone(), value.clone()))
            })
            .collect()
    }
}

#[async_trait]
impl IdentityService for KratosService {
    async fn fetch_flow<F: UiNodeVariant>(
        &self,
        headers: &HeaderMap,
        params: &HashMap<String, String>,
    ) -> IdentityResult<Response> {
        let mut target_path = "browser";
        let req_params: HashMap<String, String> = params
            .get("flow")
            .map(|flow_id| -> Result<(String, String), IdentityError> {
                self.validate_flow_id(flow_id)?;
                target_path = "flows";
                Ok(("id".to_string(), flow_id.clone()))
            })
            .transpose()?
            .into_iter()
            .collect();
        let url = self.target_to_identity(&F::FLOW_TYPE, Some(target_path), &req_params);
        let resp = self
            .client
            .get(&url)
            .headers(self.copy_forward_headers(headers, []))
            .send()
            .await?;
        Ok(resp)
    }

    async fn proxy_submit_flow(
        &self,
        flow_type: &FlowType,
        headers: &HeaderMap,
        params: &HashMap<String, String>,
        body: Bytes,
    ) -> IdentityResult<Response> {
        let url = self.target_to_identity(&flow_type, None, params);

        let resp = self
            .client
            .post(&url)
            .headers(self.copy_forward_headers(&headers, [CONTENT_TYPE]))
            .body(body)
            .send()
            .await?;

        Ok(resp)
    }

    async fn check_session(&self, headers: &HeaderMap) -> IdentityResult<KratosSession> {
        let url = format!("{}/sessions/whoami", self.public_url);
        let resp = self
            .client
            .get(&url)
            .headers(self.copy_forward_headers(headers, []))
            .send()
            .await?;

        if resp.status() == StatusCode::UNAUTHORIZED {
            return Err(IdentityError::Unauthorized(
                "No active session found".to_string(),
            ));
        }

        if !resp.status().is_success() {
            return Err(IdentityError::CheckSession(format!(
                "Failed checking session: status_code=({})",
                resp.status()
            )));
        }

        Ok(resp.json().await?)
    }

    async fn get_error(&self, id: &str) -> IdentityResult<KratosErrorContainer> {
        // Reuse the same alphanumeric + hyphen allowlist used for flow IDs.
        self.validate_error_id(id)?;

        let url = format!("{}/self-service/errors?id={}", self.public_url, id);
        let resp = self.client.get(&url).send().await?;

        if !resp.status().is_success() {
            return Err(IdentityError::RetrieveError(format!(
                "Failed to fetch error flow: {}",
                resp.status()
            )));
        }

        Ok(resp.json::<KratosErrorContainer>().await?)
    }

    async fn create_logout_flow(
        &self,
        headers: &HeaderMap,
        return_to: Option<&str>,
    ) -> IdentityResult<KratosLogoutFlow> {
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
            .headers(self.copy_forward_headers(headers, []))
            .send()
            .await?;

        if resp.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(IdentityError::Unauthorized(
                "No active session — cannot create logout flow".to_string(),
            ));
        }

        if !resp.status().is_success() {
            return Err(IdentityError::RetrieveLogout(format!(
                "Failed to create logout flow: {}",
                resp.status()
            )));
        }

        Ok(resp.json().await?)
    }
}
