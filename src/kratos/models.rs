use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UiText {
    pub id: i64,
    pub text: String,
    #[serde(rename = "type")]
    pub text_type: String,
    pub context: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UiContainer {
    pub action: String,
    pub method: String,
    pub nodes: Vec<UiNode>,
    pub messages: Option<Vec<UiText>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UiNode {
    #[serde(rename = "type")]
    pub node_type: String,
    pub group: String,
    pub attributes: UiNodeAttributes,
    pub messages: Vec<UiText>,
    pub meta: UiNodeMeta,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UiNodeMeta {
    pub label: Option<UiText>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UiNodeDivisionAttributes {
    pub id: String,
    #[serde(default)]
    pub class: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "node_type")]
pub enum UiNodeAttributes {
    #[serde(rename = "input")]
    Input(UiNodeInputAttributes),
    #[serde(rename = "img")]
    Image(UiNodeImageAttributes),
    #[serde(rename = "a")]
    Anchor(UiNodeAnchorAttributes),
    #[serde(rename = "text")]
    Text(UiNodeTextAttributes),
    #[serde(rename = "script")]
    Script(UiNodeScriptAttributes),
    #[serde(rename = "div")]
    Division(UiNodeDivisionAttributes),
    #[serde(other)]
    Unknown,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UiNodeInputAttributes {
    pub name: String,
    #[serde(rename = "type")]
    pub input_type: String,
    pub value: Option<serde_json::Value>,
    pub required: Option<bool>,
    pub disabled: bool,
    pub label: Option<UiText>,
    pub onclick: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UiNodeImageAttributes {
    pub id: String,
    pub src: String,
    pub width: i64,
    pub height: i64,
    pub alt: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UiNodeAnchorAttributes {
    pub id: String,
    pub href: String,
    pub title: UiText,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UiNodeTextAttributes {
    pub id: String,
    pub text: UiText,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UiNodeScriptAttributes {
    pub id: String,
    pub src: String,
    #[serde(rename = "type")]
    pub script_type: String,
    #[serde(rename = "async")]
    pub async_src: bool,
    pub crossorigin: String,
    pub integrity: String,
    pub referrerpolicy: String,
}

// Flow containers
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginFlow {
    pub id: String,
    #[serde(rename = "type")]
    pub flow_type: String,
    pub expires_at: String,
    pub issued_at: String,
    pub request_url: String,
    pub ui: UiContainer,
    pub active: Option<String>,
    pub return_to: Option<String>,
    pub state: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegistrationFlow {
    pub id: String,
    #[serde(rename = "type")]
    pub flow_type: String,
    pub expires_at: String,
    pub issued_at: String,
    pub request_url: String,
    pub ui: UiContainer,
    pub active: Option<String>,
    pub return_to: Option<String>,
    pub state: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RecoveryFlow {
    pub id: String,
    #[serde(rename = "type")]
    pub flow_type: String,
    pub expires_at: String,
    pub issued_at: Option<String>,
    pub request_url: Option<String>,
    pub ui: UiContainer,
    pub active: Option<String>,
    pub state: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VerificationFlow {
    pub id: String,
    #[serde(rename = "type")]
    pub flow_type: String,
    pub expires_at: String,
    pub issued_at: Option<String>,
    pub request_url: Option<String>,
    pub ui: UiContainer,
    pub active: Option<String>,
    pub state: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SettingsFlow {
    pub id: String,
    #[serde(rename = "type")]
    pub flow_type: String,
    pub expires_at: String,
    pub ui: UiContainer,
    pub active: Option<String>,
    pub identity: serde_json::Value,
    pub state: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KratosSession {
    pub id: String,
    pub active: bool,
    pub expires_at: String,
    pub authenticated_at: String,
    pub identity: KratosIdentity,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KratosIdentity {
    pub id: String,
    pub traits: serde_json::Value,
}

// ─── Error API ────────────────────────────────────────────────────────────────

/// Response from `GET /self-service/errors?id=<id>`
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KratosErrorContainer {
    pub id: String,
    pub error: KratosErrorDetail,
}

/// Inner error detail returned by the Kratos Self-Service Errors API.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KratosErrorDetail {
    /// HTTP status code (e.g. 410 for "flow expired")
    pub code: Option<i64>,
    /// Short machine-readable status (e.g. "Gone", "Forbidden")
    pub status: Option<String>,
    /// Human-readable reason phrase
    pub reason: Option<String>,
    /// Longer description of what went wrong
    pub message: Option<String>,
}

// ─── Logout API ───────────────────────────────────────────────────────────────

/// Response from `GET /self-service/logout/browser`.
/// The `logout_url` is a complete, CSRF-protected URL that the browser
/// should be redirected to in order to destroy the active Kratos session.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KratosLogoutFlow {
    pub logout_url: String,
    pub logout_token: String,
}
