use crate::hydra::models::{AcceptConsentRequest, AcceptLoginRequest, RejectRequest};
use crate::shared::error::AppError;
use crate::ui::pages::*;

use crate::kratos::models::{
    LoginFlow, RecoveryFlow, RegistrationFlow, SettingsFlow, VerificationFlow,
};
use crate::shared::state::AppState;
use axum::response::Redirect;
use axum::{
    extract::{Query, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use axum_extra::extract::Form;
use leptos::tachys::view::RenderHtml;
use std::collections::HashMap;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(|| async { Redirect::temporary("/settings") }))
        .route("/login", get(get_login).post(post_login))
        .route(
            "/registration",
            get(get_registration).post(post_registration),
        )
        .route("/recovery", get(get_recovery).post(post_recovery))
        .route(
            "/verification",
            get(get_verification).post(post_verification),
        )
        .route("/settings", get(get_settings).post(post_settings))
        .route("/oauth2/login", get(get_oauth2_login))
        .route(
            "/oauth2/consent",
            get(get_oauth2_consent).post(post_oauth2_consent),
        )
        .with_state(state)
}

fn rewrite_kratos_redirect(location: &str, kratos_url: &str, public_base: &str) -> String {
    let kratos_base = kratos_url.trim_end_matches('/');
    let public = public_base.trim_end_matches('/');

    // 1. First, replace the Kratos domain with the application's public domain
    let mut rewritten = if location.starts_with(kratos_base) {
        location.replace(kratos_base, public)
    } else {
        location.to_string()
    };

    // 2. Map default Kratos paths to the clean application routes
    let path_mappings = [
        ("/self-service/login/browser", "/login"),
        ("/self-service/registration/browser", "/registration"),
        ("/self-service/recovery/browser", "/recovery"),
        ("/self-service/verification/browser", "/verification"),
        ("/self-service/settings/browser", "/settings"),
        // Fallback mappings (in case of redirects without the /browser suffix)
        ("/self-service/login", "/login"),
        ("/self-service/registration", "/registration"),
        ("/self-service/recovery", "/recovery"),
        ("/self-service/verification", "/verification"),
        ("/self-service/settings", "/settings"),
    ];

    for (kratos_path, local_path) in path_mappings {
        // Use contains and replace to preserve any existing query parameters
        if rewritten.contains(kratos_path) {
            rewritten = rewritten.replace(kratos_path, local_path);
            break;
        }
    }

    rewritten
}

fn rewrite_action_url(action: &str, kratos_url: &str, public_base: &str) -> String {
    let kratos_base = kratos_url.trim_end_matches('/');
    let public = public_base.trim_end_matches('/');

    let rewrites = [
        ("/self-service/login", "/login"),
        ("/self-service/registration", "/registration"),
        ("/self-service/recovery", "/recovery"),
        ("/self-service/verification", "/verification"),
        ("/self-service/settings", "/settings"),
    ];

    for (kratos_path, local_path) in &rewrites {
        let prefix = format!("{}{}", kratos_base, kratos_path);
        if let Some(suffix) = action.strip_prefix(&prefix) {
            return format!("{}{}{}", public, local_path, suffix);
        }
    }

    rewrite_kratos_redirect(action, kratos_url, public_base)
}

async fn init_flow(
    flow_type: &str,
    state: &AppState,
    headers: &HeaderMap,
    params: &HashMap<String, String>,
) -> Result<Response, AppError> {
    let mut target_url = format!(
        "{}/self-service/{}/browser",
        state.config.kratos_public_url, flow_type
    );
    if !params.is_empty() {
        let query_string = serde_urlencoded::to_string(params).unwrap_or_default();
        target_url = format!("{}?{}", target_url, query_string);
    }

    let resp = state
        .kratos
        .client
        .get(&target_url)
        .headers(state.kratos.copy_forward_headers(headers))
        .send()
        .await?;

    let mut response_headers = HeaderMap::new();
    for cookie in resp.headers().get_all(reqwest::header::SET_COOKIE) {
        response_headers.append(http::header::SET_COOKIE, cookie.clone());
    }

    if resp.status().is_redirection() {
        if let Some(location) = resp.headers().get(reqwest::header::LOCATION) {
            let loc_str = location.to_str().unwrap_or_default();
            let local_location = rewrite_kratos_redirect(
                loc_str,
                &state.config.kratos_public_url,
                &state.config.public_base_url,
            );
            if let Ok(val) = HeaderValue::from_str(&local_location) {
                response_headers.insert(http::header::LOCATION, val);
            }
            return Ok((resp.status(), response_headers).into_response());
        }
    }

    let status = resp.status();

    if status.is_success() {
        let body_text: String = resp.text().await?;
        match flow_type {
            "login" => {
                if let Ok(mut flow) = serde_json::from_str::<LoginFlow>(&body_text) {
                    flow.ui.action = rewrite_action_url(
                        &flow.ui.action,
                        &state.config.kratos_public_url,
                        &state.config.public_base_url,
                    );
                    let html = leptos::view! { <LoginPage flow=flow /> }.to_html();
                    return Ok((
                        StatusCode::OK,
                        response_headers,
                        [("content-type", "text/html; charset=utf-8")],
                        html,
                    )
                        .into_response());
                }
            }
            "registration" => {
                if let Ok(mut flow) = serde_json::from_str::<RegistrationFlow>(&body_text) {
                    flow.ui.action = rewrite_action_url(
                        &flow.ui.action,
                        &state.config.kratos_public_url,
                        &state.config.public_base_url,
                    );
                    let html = leptos::view! { <RegistrationPage flow=flow /> }.to_html();
                    return Ok((
                        StatusCode::OK,
                        response_headers,
                        [("content-type", "text/html; charset=utf-8")],
                        html,
                    )
                        .into_response());
                }
            }
            "recovery" => {
                if let Ok(mut flow) = serde_json::from_str::<RecoveryFlow>(&body_text) {
                    flow.ui.action = rewrite_action_url(
                        &flow.ui.action,
                        &state.config.kratos_public_url,
                        &state.config.public_base_url,
                    );
                    let html = leptos::view! { <RecoveryPage flow=flow /> }.to_html();
                    return Ok((
                        StatusCode::OK,
                        response_headers,
                        [("content-type", "text/html; charset=utf-8")],
                        html,
                    )
                        .into_response());
                }
            }
            "verification" => {
                if let Ok(mut flow) = serde_json::from_str::<VerificationFlow>(&body_text) {
                    flow.ui.action = rewrite_action_url(
                        &flow.ui.action,
                        &state.config.kratos_public_url,
                        &state.config.public_base_url,
                    );
                    let html = leptos::view! { <VerificationPage flow=flow /> }.to_html();
                    return Ok((
                        StatusCode::OK,
                        response_headers,
                        [("content-type", "text/html; charset=utf-8")],
                        html,
                    )
                        .into_response());
                }
            }
            "settings" => {
                if let Ok(mut flow) = serde_json::from_str::<SettingsFlow>(&body_text) {
                    flow.ui.action = rewrite_action_url(
                        &flow.ui.action,
                        &state.config.kratos_public_url,
                        &state.config.public_base_url,
                    );
                    let html = leptos::view! { <SettingsPage flow=flow /> }.to_html();
                    return Ok((
                        StatusCode::OK,
                        response_headers,
                        [("content-type", "text/html; charset=utf-8")],
                        html,
                    )
                        .into_response());
                }
            }
            _ => {}
        }
    }

    Err(AppError::Kratos(format!(
        "Failed to initialize {} flow: status {}",
        flow_type, status
    )))
}

async fn proxy_post_to_kratos(
    flow_type: &str,
    state: AppState,
    headers: HeaderMap,
    params: HashMap<String, String>,
    body_bytes: axum::body::Bytes,
) -> Result<Response, AppError> {
    let mut target_url = format!(
        "{}/self-service/{}",
        state.config.kratos_public_url, flow_type
    );
    if !params.is_empty() {
        let query_string = serde_urlencoded::to_string(&params).unwrap_or_default();
        target_url = format!("{}?{}", target_url, query_string);
    }

    let mut req_builder = state
        .kratos
        .client
        .post(&target_url)
        .headers(state.kratos.copy_forward_headers(&headers));

    if let Some(content_type) = headers.get(reqwest::header::CONTENT_TYPE) {
        req_builder = req_builder.header(reqwest::header::CONTENT_TYPE, content_type.clone());
    }

    let resp = req_builder.body(body_bytes).send().await?;

    let mut response_headers = HeaderMap::new();
    for cookie in resp.headers().get_all(reqwest::header::SET_COOKIE) {
        response_headers.append(http::header::SET_COOKIE, cookie.clone());
    }

    if resp.status().is_redirection() {
        if let Some(location) = resp.headers().get(reqwest::header::LOCATION) {
            let loc_str = location.to_str().unwrap_or_default();
            let local_location = rewrite_kratos_redirect(
                loc_str,
                &state.config.kratos_public_url,
                &state.config.public_base_url,
            );
            if let Ok(val) = HeaderValue::from_str(&local_location) {
                response_headers.insert(http::header::LOCATION, val);
            }
            return Ok((resp.status(), response_headers).into_response());
        }
    }

    let status = resp.status();
    let body_text: String = resp.text().await?;

    if status.is_success() || status == StatusCode::BAD_REQUEST {
        match flow_type {
            "login" => {
                if let Ok(mut flow) = serde_json::from_str::<LoginFlow>(&body_text) {
                    flow.ui.action = rewrite_action_url(
                        &flow.ui.action,
                        &state.config.kratos_public_url,
                        &state.config.public_base_url,
                    );
                    let html = leptos::view! { <LoginPage flow=flow /> }.to_html();
                    return Ok((
                        status,
                        response_headers,
                        [("content-type", "text/html; charset=utf-8")],
                        html,
                    )
                        .into_response());
                }
            }
            "registration" => {
                if let Ok(mut flow) = serde_json::from_str::<RegistrationFlow>(&body_text) {
                    flow.ui.action = rewrite_action_url(
                        &flow.ui.action,
                        &state.config.kratos_public_url,
                        &state.config.public_base_url,
                    );
                    let html = leptos::view! { <RegistrationPage flow=flow /> }.to_html();
                    return Ok((
                        status,
                        response_headers,
                        [("content-type", "text/html; charset=utf-8")],
                        html,
                    )
                        .into_response());
                }
            }
            "recovery" => {
                if let Ok(mut flow) = serde_json::from_str::<RecoveryFlow>(&body_text) {
                    flow.ui.action = rewrite_action_url(
                        &flow.ui.action,
                        &state.config.kratos_public_url,
                        &state.config.public_base_url,
                    );
                    let html = leptos::view! { <RecoveryPage flow=flow /> }.to_html();
                    return Ok((
                        status,
                        response_headers,
                        [("content-type", "text/html; charset=utf-8")],
                        html,
                    )
                        .into_response());
                }
            }
            "verification" => {
                if let Ok(mut flow) = serde_json::from_str::<VerificationFlow>(&body_text) {
                    flow.ui.action = rewrite_action_url(
                        &flow.ui.action,
                        &state.config.kratos_public_url,
                        &state.config.public_base_url,
                    );
                    let html = leptos::view! { <VerificationPage flow=flow /> }.to_html();
                    return Ok((
                        status,
                        response_headers,
                        [("content-type", "text/html; charset=utf-8")],
                        html,
                    )
                        .into_response());
                }
            }
            "settings" => {
                if let Ok(mut flow) = serde_json::from_str::<SettingsFlow>(&body_text) {
                    flow.ui.action = rewrite_action_url(
                        &flow.ui.action,
                        &state.config.kratos_public_url,
                        &state.config.public_base_url,
                    );
                    let html = leptos::view! { <SettingsPage flow=flow /> }.to_html();
                    return Ok((
                        status,
                        response_headers,
                        [("content-type", "text/html; charset=utf-8")],
                        html,
                    )
                        .into_response());
                }
            }
            _ => {}
        }
    }

    tracing::error!(
        "Kratos form submission failed: status={}, body={}",
        status,
        body_text
    );
    Err(AppError::Kratos(format!(
        "Form submission failed (status {})",
        status
    )))
}

async fn get_login(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Response, AppError> {
    if let Some(flow_id) = params.get("flow") {
        let (mut flow, set_cookie_headers) = state.kratos.get_login_flow(flow_id, &headers).await?;
        flow.ui.action = rewrite_action_url(
            &flow.ui.action,
            &state.config.kratos_public_url,
            &state.config.public_base_url,
        );

        let html = leptos::view! { <LoginPage flow=flow /> }.to_html();
        return Ok((
            StatusCode::OK,
            set_cookie_headers,
            [("content-type", "text/html; charset=utf-8")],
            html,
        )
            .into_response());
    }

    init_flow("login", &state, &headers, &params).await
}

async fn post_login(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
    body: axum::body::Bytes,
) -> Result<Response, AppError> {
    proxy_post_to_kratos("login", state, headers, params, body).await
}

async fn get_registration(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Response, AppError> {
    if let Some(flow_id) = params.get("flow") {
        let (mut flow, set_cookie_headers) = state
            .kratos
            .get_registration_flow(flow_id, &headers)
            .await?;
        flow.ui.action = rewrite_action_url(
            &flow.ui.action,
            &state.config.kratos_public_url,
            &state.config.public_base_url,
        );

        let html = leptos::view! { <RegistrationPage flow=flow /> }.to_html();
        return Ok((
            StatusCode::OK,
            set_cookie_headers,
            [("content-type", "text/html; charset=utf-8")],
            html,
        )
            .into_response());
    }

    init_flow("registration", &state, &headers, &params).await
}

async fn post_registration(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
    body: axum::body::Bytes,
) -> Result<Response, AppError> {
    proxy_post_to_kratos("registration", state, headers, params, body).await
}

async fn get_recovery(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Response, AppError> {
    if let Some(flow_id) = params.get("flow") {
        let (mut flow, set_cookie_headers) =
            state.kratos.get_recovery_flow(flow_id, &headers).await?;
        flow.ui.action = rewrite_action_url(
            &flow.ui.action,
            &state.config.kratos_public_url,
            &state.config.public_base_url,
        );

        let html = leptos::view! { <RecoveryPage flow=flow /> }.to_html();
        return Ok((
            StatusCode::OK,
            set_cookie_headers,
            [("content-type", "text/html; charset=utf-8")],
            html,
        )
            .into_response());
    }

    init_flow("recovery", &state, &headers, &params).await
}

async fn post_recovery(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
    body: axum::body::Bytes,
) -> Result<Response, AppError> {
    proxy_post_to_kratos("recovery", state, headers, params, body).await
}

async fn get_verification(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Response, AppError> {
    if let Some(flow_id) = params.get("flow") {
        let (mut flow, set_cookie_headers) = state
            .kratos
            .get_verification_flow(flow_id, &headers)
            .await?;
        flow.ui.action = rewrite_action_url(
            &flow.ui.action,
            &state.config.kratos_public_url,
            &state.config.public_base_url,
        );

        let html = leptos::view! { <VerificationPage flow=flow /> }.to_html();
        return Ok((
            StatusCode::OK,
            set_cookie_headers,
            [("content-type", "text/html; charset=utf-8")],
            html,
        )
            .into_response());
    }

    init_flow("verification", &state, &headers, &params).await
}

async fn post_verification(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
    body: axum::body::Bytes,
) -> Result<Response, AppError> {
    proxy_post_to_kratos("verification", state, headers, params, body).await
}

async fn get_settings(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Response, AppError> {
    if let Some(flow_id) = params.get("flow") {
        let (mut flow, set_cookie_headers) =
            state.kratos.get_settings_flow(flow_id, &headers).await?;
        flow.ui.action = rewrite_action_url(
            &flow.ui.action,
            &state.config.kratos_public_url,
            &state.config.public_base_url,
        );

        let html = leptos::view! { <SettingsPage flow=flow /> }.to_html();
        return Ok((
            StatusCode::OK,
            set_cookie_headers,
            [("content-type", "text/html; charset=utf-8")],
            html,
        )
            .into_response());
    }

    init_flow("settings", &state, &headers, &params).await
}

async fn post_settings(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
    body: axum::body::Bytes,
) -> Result<Response, AppError> {
    proxy_post_to_kratos("settings", state, headers, params, body).await
}

async fn get_oauth2_login(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Response, AppError> {
    let challenge = params
        .get("login_challenge")
        .ok_or_else(|| AppError::Hydra("Missing login_challenge parameter".to_string()))?;

    let login_req = state.hydra.get_login_request(challenge).await?;
    if login_req.skip {
        let accept_res = state
            .hydra
            .accept_login_request(
                challenge,
                AcceptLoginRequest {
                    subject: login_req.subject.ok_or_else(|| {
                        AppError::Hydra(
                            "Missing subject in login request despite skip=true".to_string(),
                        )
                    })?,
                    remember: Some(true),
                    remember_for: Some(3600),
                    acr: None,
                    context: None,
                },
            )
            .await?;
        return Ok((
            StatusCode::SEE_OTHER,
            axum::response::Redirect::to(&accept_res.redirect_to),
        )
            .into_response());
    }

    match state.kratos.check_session(&headers).await {
        Ok(session) => {
            let accept_res = state
                .hydra
                .accept_login_request(
                    challenge,
                    AcceptLoginRequest {
                        subject: session.identity.id,
                        remember: Some(true),
                        remember_for: Some(3600),
                        acr: None,
                        context: None,
                    },
                )
                .await?;
            Ok((
                StatusCode::SEE_OTHER,
                axum::response::Redirect::to(&accept_res.redirect_to),
            )
                .into_response())
        }
        Err(_) => {
            let encoded_challenge =
                serde_urlencoded::to_string(&[("login_challenge", challenge)]).unwrap_or_default();
            let redirect_url = format!("/login?{}", encoded_challenge);
            Ok((
                StatusCode::SEE_OTHER,
                axum::response::Redirect::to(&redirect_url),
            )
                .into_response())
        }
    }
}

async fn get_oauth2_consent(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Response, AppError> {
    let challenge = params
        .get("consent_challenge")
        .ok_or_else(|| AppError::Hydra("Missing consent_challenge parameter".to_string()))?;

    let consent_req = state.hydra.get_consent_request(challenge).await?;

    if consent_req.skip.unwrap_or(false) {
        let accept_res = state
            .hydra
            .accept_consent_request(
                challenge,
                AcceptConsentRequest {
                    grant_scope: consent_req.requested_scope,
                    grant_access_token_audience: consent_req.requested_access_token_audience,
                    remember: Some(true),
                    remember_for: Some(3600),
                    session: None,
                },
            )
            .await?;
        return Ok((
            StatusCode::SEE_OTHER,
            axum::response::Redirect::to(&accept_res.redirect_to),
        )
            .into_response());
    }

    let html = leptos::view! { <ConsentPage req=consent_req /> }.to_html();

    Ok((
        StatusCode::OK,
        [("content-type", "text/html; charset=utf-8")],
        html,
    )
        .into_response())
}

#[derive(serde::Deserialize)]
pub struct ConsentForm {
    pub consent_challenge: String,
    pub submit: String,
    #[serde(rename = "grant_scope[]")]
    pub grant_scope: Option<Vec<String>>,
}

async fn post_oauth2_consent(
    State(state): State<AppState>,
    headers: HeaderMap,
    Form(form): Form<ConsentForm>,
) -> Result<Response, AppError> {
    if form.submit == "accept" {
        let grant_scope = form.grant_scope.unwrap_or_default();

        // Fetch active user session from Kratos
        let kratos_session = match state.kratos.check_session(&headers).await {
            Ok(session) => session,
            Err(_) => {
                return Err(AppError::Unauthorized("No active Kratos session found during consent".to_string()));
            }
        };

        let email = kratos_session
            .identity
            .traits
            .get("email")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();

        let subject = kratos_session.identity.id;

        // Populate session claims for downstream OIDC clients (like oauth2-proxy)
        let session = Some(crate::hydra::models::ConsentRequestSession {
            id_token: Some(serde_json::json!({
                "sub": subject,
                "email": email,
            })),
            access_token: None,
        });

        let accept_res = state
            .hydra
            .accept_consent_request(
                &form.consent_challenge,
                AcceptConsentRequest {
                    grant_scope,
                    grant_access_token_audience: None,
                    remember: Some(true),
                    remember_for: Some(3600),
                    session,
                },
            )
            .await?;

        Ok((
            StatusCode::SEE_OTHER,
            Redirect::to(&accept_res.redirect_to),
        )
            .into_response())
    } else {
        // Handle consent rejection
        let reject_res = state
            .hydra
            .reject_consent_request(
                &form.consent_challenge,
                RejectRequest {
                    error: "consent_denied".to_string(),
                    error_description: Some("The user denied the consent request.".to_string()),
                    error_uri: None,
                    status_code: Some(StatusCode::FORBIDDEN.as_u16() as i64),
                },
            )
            .await?;

        Ok((StatusCode::SEE_OTHER, Redirect::to(&reject_res.redirect_to)).into_response())
    }
}
