use crate::core::domain::oidc_domains::OAuth2Client;
use crate::core::ports::flow::{SubmitNodeVariant, UiNodeVariant};
use http::{HeaderMap, StatusCode};
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Display, Clone, Eq, Hash, PartialEq)]
pub enum FlowType {
    Login,
    Registration,
    Recovery,
    Verification,
    Settings,
    Error,
    Logout,
    Consent,
}

pub struct UiNodeResponse<U: UiNodeVariant> {
    pub ui_node: Option<U>,
    pub status_code: StatusCode,
    pub headers: HeaderMap,
}

pub struct ActionResponse {
    pub status_code: StatusCode,
    pub headers: HeaderMap,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginUiNode {
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
impl UiNodeVariant for LoginUiNode {
    const FLOW_TYPE: FlowType = FlowType::Login;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegistrationUiNode {
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
impl UiNodeVariant for RegistrationUiNode {
    const FLOW_TYPE: FlowType = FlowType::Registration;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RecoveryUiNode {
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
impl UiNodeVariant for RecoveryUiNode {
    const FLOW_TYPE: FlowType = FlowType::Recovery;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VerificationUiNode {
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
impl UiNodeVariant for VerificationUiNode {
    const FLOW_TYPE: FlowType = FlowType::Verification;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SettingsUiNode {
    pub id: String,
    #[serde(rename = "type")]
    pub flow_type: String,
    pub expires_at: String,
    pub ui: UiContainer,
    pub active: Option<String>,
    pub identity: serde_json::Value,
    pub state: Option<String>,
}
impl UiNodeVariant for SettingsUiNode {
    const FLOW_TYPE: FlowType = FlowType::Settings;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConsentUiNode {
    pub challenge: String,
    pub subject: Option<String>,
    pub skip: Option<bool>,
    pub client: OAuth2Client,
    pub requested_scope: Vec<String>,
    pub requested_access_token_audience: Option<Vec<String>>,
    pub login_session_id: Option<String>,
}
impl UiNodeVariant for ConsentUiNode {
    const FLOW_TYPE: FlowType = FlowType::Consent;
}

#[derive(serde::Deserialize)]
pub struct ConsentSubmitNode {
    pub consent_challenge: String,
    pub submit: String,
    #[serde(rename = "grant_scope[]")]
    pub grant_scope: Option<Vec<String>>,
}
impl SubmitNodeVariant for ConsentSubmitNode {
    const FLOW_TYPE: FlowType = FlowType::Consent;
}

#[derive(Clone, Debug, Deserialize)]
pub struct ErrorUiNode {
    pub title: String,
    pub description: String,
}
impl UiNodeVariant for ErrorUiNode {
    const FLOW_TYPE: FlowType = FlowType::Consent;
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KratosErrorContainer {
    pub id: String,
    pub error: KratosErrorDetail,
}

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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KratosLogoutFlow {
    pub logout_url: String,
    pub logout_token: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UiContainer {
    pub action: String,
    pub method: String,
    pub nodes: Vec<UiNode>,
    pub messages: Option<Vec<UiText>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UiText {
    pub id: i64,
    pub text: String,
    #[serde(rename = "type")]
    pub text_type: String,
    pub context: Option<serde_json::Value>,
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