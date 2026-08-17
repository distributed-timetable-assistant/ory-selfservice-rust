use axum::response::{IntoResponse, Response};
use leptos::prelude::*;
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Kratos Client Error: {0}")]
    Kratos(String),

    #[error("Hydra Client Error: {0}")]
    Hydra(String),

    #[error("Serialization Error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Network Error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Internal Server Error: {0}")]
    Internal(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Kratos(_) | AppError::Hydra(_) => StatusCode::BAD_GATEWAY,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        // Log detail server-side
        tracing::error!("AppError occurred: {:?}", self);

        // Sanitize message for user display
        let message = match &self {
            AppError::Unauthorized(msg) => msg.clone(),
            AppError::NotFound(msg) => msg.clone(),
            AppError::Kratos(_) => {
                "Upstream Ory Kratos service error occurred. Please try again.".to_string()
            }
            AppError::Hydra(_) => {
                "Upstream Ory Hydra service error occurred. Please try again.".to_string()
            }
            _ => "An internal server error occurred.".to_string(),
        };

        use http::StatusCode;
        use leptos::tachys::view::RenderHtml;

        // Render error page using Leptos
        let html = view! {
            <!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="UTF-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                    <title>"Error - Ory Shield UI"</title>
                    <style>
                        "body {
                            background-color: #0f172a;
                            color: #f8fafc;
                            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
                            display: flex;
                            align-items: center;
                            justify-content: center;
                            min-height: 100vh;
                            margin: 0;
                        }
                        .card {
                            background-color: #1e293b;
                            padding: 2rem;
                            border-radius: 0.75rem;
                            box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1);
                            max-width: 28rem;
                            width: 100%;
                            text-align: center;
                            border: 1px solid #334155;
                        }
                        h1 { color: #f43f5e; margin-top: 0; }
                        p { color: #94a3b8; line-height: 1.5; }
                        .btn {
                            display: inline-block;
                            margin-top: 1.5rem;
                            padding: 0.5rem 1rem;
                            background-color: #3b82f6;
                            color: white;
                            text-decoration: none;
                            border-radius: 0.375rem;
                            font-weight: 500;
                        }
                        .btn:hover { background-color: #2563eb; }"
                    </style>
                </head>
                <body>
                    <div class="card">
                        <h1>"An Error Occurred"</h1>
                        <p>{message.clone()}</p>
                        <a class="btn" href="/login">"Back to Login"</a>
                    </div>
                </body>
            </html>
        }.to_html();

        Response::builder()
            .status(status)
            .header("content-type", "text/html; charset=utf-8")
            .body(axum::body::Body::from(html))
            .unwrap_or_else(|_| {
                (status, format!("Internal Server Error: {}", message)).into_response()
            })
    }
}
