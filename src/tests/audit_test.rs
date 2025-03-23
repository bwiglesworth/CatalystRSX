#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use catalyst_rsx::logging::audit::{AuditLogger, AuditEvent, AuditEventType, AuditSeverity};

    #[test]
    fn test_audit_logging() {
        let temp_dir = std::env::temp_dir();
        let log_path = temp_dir.join("test_audit.log");
        
        let mut logger = AuditLogger::new(log_path.clone()).unwrap();
        
        let event = AuditEvent {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            event_type: AuditEventType::Authentication,
            severity: AuditSeverity::Warning,
            details: "Failed login attempt".to_string(),
            source_ip: "127.0.0.1".to_string(),
            user_agent: Some("test-client".to_string()),
        };

        logger.log_event(event).unwrap();

        // Verify log file exists and contains data
        let log_contents = std::fs::read_to_string(log_path).unwrap();
        assert!(log_contents.contains("Failed login attempt"));
        assert!(log_contents.contains("Authentication"));
    }
}
