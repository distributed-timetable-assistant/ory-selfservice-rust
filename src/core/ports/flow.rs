use crate::core::domain::error::FlowResult;
use crate::core::domain::flow::{ActionResponse, FlowType, UiNodeResponse};
use async_trait::async_trait;
use axum::body::Bytes;
use http::HeaderMap;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

pub trait UiNodeVariant: DeserializeOwned {
    const FLOW_TYPE: FlowType;
}

pub trait SubmitNodeVariant: DeserializeOwned {
    const FLOW_TYPE: FlowType;
}

#[async_trait]
pub trait FetchUiFlowHandler<U: UiNodeVariant>: Send + Sync {
    async fn fetch_ui(
        &self,
        headers: &HeaderMap,
        params: &HashMap<String, String>,
    ) -> FlowResult<UiNodeResponse<U>>;
}

#[async_trait]
pub trait SubmitUiFlowHandler<S: SubmitNodeVariant, U: UiNodeVariant>: Send + Sync {
    async fn submit_ui(
        &self,
        headers: &HeaderMap,
        params: &HashMap<String, String>,
        submit: S,
    ) -> FlowResult<UiNodeResponse<U>>;
}

#[async_trait]
pub trait ProxySubmitUiFlowHandler<U: UiNodeVariant>: Send + Sync {
    async fn proxy_submit_ui(
        &self,
        headers: &HeaderMap,
        params: &HashMap<String, String>,
        body: Bytes,
    ) -> FlowResult<UiNodeResponse<U>>;
}

#[async_trait]
pub trait FetchActionFlowHandler: Send + Sync {
    async fn fetch_action(
        &self,
        headers: &HeaderMap,
        params: &HashMap<String, String>,
    ) -> FlowResult<ActionResponse>;
}

#[async_trait]
pub trait SubmitActionFlowHandler<S: SubmitNodeVariant>: Send + Sync {
    async fn submit_action(
        &self,
        headers: &HeaderMap,
        params: &HashMap<String, String>,
        submit: S,
    ) -> FlowResult<ActionResponse>;
}
