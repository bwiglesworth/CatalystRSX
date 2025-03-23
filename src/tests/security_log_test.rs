#[cfg(test)]
mod tests {
    use catalyst_rsx::logging::security::{SecurityLogger, SecurityLogLevel};

    #[test]
    fn test_security_logging() {
        let temp_dir = std::env::temp_dir();
        let log_path = temp_dir.join("security_test.log");
        let logger = SecurityLogger::new(SecurityLogLevel::High, log_path).unwrap();
        
        assert_eq!(logger.get_log_level(), SecurityLogLevel::High);
    }
}
