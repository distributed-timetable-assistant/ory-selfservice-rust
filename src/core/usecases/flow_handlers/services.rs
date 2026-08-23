use crate::core::domain::error::{IdentityError, IdentityResult};
use crate::core::domain::flow::UiNodeResponse;
use crate::core::ports::flow::UiNodeVariant;
use crate::core::ports::services::identity::IdentityService;
use crate::core::ports::utils::url_rewrite::UrlRewriter;
use axum::body::Bytes;
use http::header::{LOCATION, SET_COOKIE};
use http::{HeaderMap, HeaderValue, StatusCode};
use std::collections::HashMap;
use std::sync::Arc;

pub async fn handle_identity_fetch_flow<F: UiNodeVariant>(
    headers: &HeaderMap,
    params: &HashMap<String, String>,
    identity_service: Arc<impl IdentityService>,
    url_rewriter: Arc<impl UrlRewriter>,
) -> IdentityResult<UiNodeResponse<F>> {
    let resp = identity_service.fetch_flow::<F>(headers, params).await?;
    let status = resp.status();
    if !status.is_success() || params.get("flow").is_some() && status.is_redirection() {
        return Err(IdentityError::RetrieveRequest(format!(
            "Failed to get {} flow: {}",
            &F::FLOW_TYPE,
            status
        )));
    }

    let mut resp_headers: HeaderMap = resp
        .headers()
        .get_all(SET_COOKIE)
        .iter()
        .map(|cookie| (SET_COOKIE, cookie.clone()))
        .collect();

    if status.is_redirection() {
        headers
            .get_all(LOCATION)
            .into_iter()
            .fold(&mut resp_headers, |headers, value| {
                headers.append(LOCATION, value.clone());
                headers
            });
        return Ok(UiNodeResponse {
            ui_node: None,
            status_code: status,
            headers: rewrite_location_headers(url_rewriter, &resp_headers),
        });
    }

    Ok(UiNodeResponse {
        ui_node: Some(resp.json().await?),
        status_code: status,
        headers: resp_headers,
    })
}

fn rewrite_location_headers(url_rewriter: Arc<impl UrlRewriter>, headers: &HeaderMap) -> HeaderMap {
    let mut header_map = headers.clone();
    let locations: Vec<HeaderValue> = headers
        .get_all(LOCATION)
        .iter()
        .flat_map(|header| url_rewriter.rewrite_header(&header))
        .collect();
    header_map.remove(LOCATION);
    locations
        .into_iter()
        .fold(&mut header_map, |header_map, value| {
            header_map.append(LOCATION, value);
            header_map
        });
    header_map
}

pub async fn handle_identity_submit_flow<U: UiNodeVariant>(
    headers: &HeaderMap,
    params: &HashMap<String, String>,
    body: Bytes,
    identity_service: Arc<impl IdentityService>,
    url_rewriter: Arc<impl UrlRewriter>,
) -> IdentityResult<UiNodeResponse<U>> {
    let resp = identity_service
        .proxy_submit_flow(&U::FLOW_TYPE, headers, params, body)
        .await?;

    let status = resp.status();
    if !(status.is_success() && status == StatusCode::BAD_REQUEST && status.is_redirection()) {
        let body_text: String = resp.text().await?;
        return Err(IdentityError::PostFlow(format!(
            "Form submission failed. flow=({}), status=({}), , body=({})",
            &U::FLOW_TYPE,
            status,
            body_text
        )));
    }

    let mut resp_headers: HeaderMap = resp
        .headers()
        .get_all(SET_COOKIE)
        .iter()
        .map(|cookie| (SET_COOKIE, cookie.clone()))
        .collect();

    if status.is_redirection() {
        headers
            .get_all(LOCATION)
            .into_iter()
            .fold(&mut resp_headers, |headers, value| {
                headers.append(LOCATION, value.clone());
                headers
            });
        return Ok(UiNodeResponse {
            ui_node: None,
            status_code: status,
            headers: rewrite_location_headers(url_rewriter, &resp_headers),
        });
    }

    Ok(UiNodeResponse {
        ui_node: Some(resp.json().await?),
        status_code: status,
        headers: resp_headers,
    })
}
