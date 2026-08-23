use http::HeaderValue;

pub trait UrlRewriter: Send + Sync {
    fn rewrite(&self, url: &str) -> String;
    fn rewrite_header(&self, header: &HeaderValue) -> Option<HeaderValue>;
}
