#[cfg(test)]
mod tests {
    use catalyst_rsx::security::xss::sanitize::ContentSanitizer;    
    #[test]
    fn test_basic_sanitization() {
        let sanitizer = ContentSanitizer::new();
        let input = r#"<p>Hello</p><script>alert('xss')</script>"#;
        let result = sanitizer.sanitize(input);
        assert_eq!(result, "<p>Hello</p>");
    }
}
