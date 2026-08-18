use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OAuth2Client {
    pub client_id: String,
    pub client_name: Option<String>,
    pub logo_uri: Option<String>,
    pub policy_uri: Option<String>,
    pub tos_uri: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub challenge: String,
    pub subject: Option<String>,
    pub skip: bool,
    pub client: OAuth2Client,
    pub request_url: String,
    pub oidc_context: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AcceptLoginRequest {
    pub subject: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remember: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remember_for: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RejectRequest {
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompletedRequest {
    pub redirect_to: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConsentRequest {
    pub challenge: String,
    pub subject: Option<String>,
    pub skip: Option<bool>,
    pub client: OAuth2Client,
    pub requested_scope: Vec<String>,
    pub requested_access_token_audience: Option<Vec<String>>,
    pub login_session_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConsentRequestSession {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AcceptConsentRequest {
    pub grant_scope: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_access_token_audience: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remember: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remember_for: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<ConsentRequestSession>,
}

// ─── Logout API ───────────────────────────────────────────────────────────────

/// Response from `GET /oauth2/auth/requests/logout?logout_challenge=<challenge>`.
/// Hydra provides this during RP-Initiated Logout so the app can verify and
/// accept/reject the request before the session is torn down.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogoutRequest {
    pub challenge: String,
    /// The Kratos subject (identity ID) whose session is being logged out.
    pub subject: Option<String>,
    /// The Kratos session ID, if Hydra tracked it during the original login.
    pub sid: Option<String>,
    /// The original logout request URL (from the RP).
    pub request_url: Option<String>,
}

