#[cfg(test)]
mod tests {
    use crate::security::xss::sanitize::ContentSanitizer;    
    #[test]
    fn test_basic_sanitization() {
        let sanitizer = ContentSanitizer::new();
        let input = r#"<p>Hello</p><script>alert('xss')</script>"#;
        let result = sanitizer.sanitize(input);
        assert_eq!(result, "<p>Hello</p>");
    }

    #[test]
    fn test_event_handler_injection() {
        let sanitizer = ContentSanitizer::new();
        let input = r#"<img src="x" onerror="alert('xss')">"#;
        let result = sanitizer.sanitize(input);
        assert_eq!(result, "");
    }

    #[test]
    fn test_data_uri_injection() {
        let sanitizer = ContentSanitizer::new();
        let input = r#"<a href="data:text/html;base64,PHNjcmlwdD5hbGVydCgneHNzJyk8L3NjcmlwdD4=">click</a>"#;
        let result = sanitizer.sanitize(input);
        assert_eq!(result, "<a>click</a>");
    }
}
