use http::HeaderValue;
use url::Url;
use crate::core::ports::utils::url_rewrite::UrlRewriter;

pub struct AppInTheMiddle {
    app_url: Url,
    kratos_url: Url,
}
impl AppInTheMiddle {
    pub fn new (app_url: Url, kratos_url: Url) -> Self {
        Self { app_url, kratos_url }
    }
}
impl UrlRewriter for AppInTheMiddle {
    fn rewrite(&self, url: &str) -> String {
        rewrite_action_url(url, self.kratos_url.as_str(), self.app_url.as_str())
    }

    fn rewrite_header(&self, header: &HeaderValue) -> Option<HeaderValue> {
        let value = header.to_str().unwrap_or_default();
        let rewritten = self.rewrite(value);
        HeaderValue::from_str(&rewritten).ok()
    }
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

pub fn rewrite_kratos_redirect(location: &str, kratos_url: &str, public_base: &str) -> String {
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
