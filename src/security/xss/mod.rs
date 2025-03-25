pub mod sanitize;pub use sanitize::ContentSanitizer;

pub struct XssMiddleware;

impl XssMiddleware {
    pub fn new() -> Self {
        XssMiddleware
    }

    pub fn sanitize_html(content: &str) -> String {
        content
            .replace("&", "&")
            .replace("<", "<")
            .replace(">", ">")
    }
}