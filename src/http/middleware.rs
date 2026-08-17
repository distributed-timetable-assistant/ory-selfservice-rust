use axum::{
    body::Body,
    http::{Request, Response},
    middleware::Next,
};
use tracing::{info, span, Level};

pub async fn trace_request_context(
    req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, std::convert::Infallible> {
    let method = req.method().clone();
    let uri = req.uri().clone();

    // Get client IP
    let client_ip = req
        .headers()
        .get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .or_else(|| req.headers().get("x-real-ip").and_then(|h| h.to_str().ok()))
        .unwrap_or("unknown")
        .to_string();

    let user_agent = req
        .headers()
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    let span = span!(
        Level::INFO,
        "http_request",
        method = %method,
        uri = %uri,
        client_ip = %client_ip,
        user_agent = %user_agent
    );

    let _enter = span.enter();
    info!("Incoming request");

    let start = std::time::Instant::now();
    let response = next.run(req).await;
    let duration = start.elapsed();

    info!(
        status = %response.status().as_u16(),
        duration_ms = duration.as_millis(),
        "Finished request processing"
    );

    Ok(response)
}
