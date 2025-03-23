use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use std::sync::Mutex;
use actix_web::dev::ServiceRequest;
use crate::logging::audit::{AuditLogger, AuditEvent, AuditEventType, AuditSeverity};
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SecurityLogLevel {
    High,
    Medium,
    Low,
}
struct ActivityTracker {
    failed_attempts: Mutex<HashMap<String, Vec<u64>>>,
    threshold: usize,
    window_secs: u64,
}

impl ActivityTracker {
    fn new(threshold: usize, window_secs: u64) -> Self {
        ActivityTracker {
            failed_attempts: Mutex::new(HashMap::new()),
            threshold,
            window_secs,
        }
    }
}

struct AuthTracker {
    attempts: Mutex<Vec<AuthAttempt>>,
    max_entries: usize,
}

impl AuthTracker {
    fn new(max_entries: usize) -> Self {
        AuthTracker {
            attempts: Mutex::new(Vec::with_capacity(max_entries)),
            max_entries,
        }
    }

    fn add_attempt(&self, attempt: AuthAttempt) {
        let mut attempts = self.attempts.lock().unwrap();
        attempts.push(attempt);
        if attempts.len() > self.max_entries {
            attempts.remove(0);
        }
    }
}
#[derive(Clone)]
pub struct AuthAttempt {
    pub timestamp: u64,
    pub ip: String,
    pub path: String,
    pub success: bool,
    pub user_agent: String,
}

pub struct SecurityLogger {
    log_level: SecurityLogLevel,
    audit_logger: AuditLogger,
    activity_tracker: ActivityTracker,
    auth_tracker: AuthTracker,
}

impl SecurityLogger {
    pub fn new(level: SecurityLogLevel, log_path: std::path::PathBuf) -> std::io::Result<Self> {
        Ok(SecurityLogger {
            log_level: level,
            audit_logger: AuditLogger::new(log_path)?,
            activity_tracker: ActivityTracker::new(5, 300),
            auth_tracker: AuthTracker::new(1000),
        })
    }

    pub fn track_auth_attempt(&mut self, req: &ServiceRequest, success: bool) {
        let attempt = AuthAttempt {
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            ip: req.connection_info().peer_addr().unwrap_or("unknown").to_string(),
            path: req.path().to_string(),
            success,
            user_agent: req.headers()
                .get("User-Agent")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("unknown")
                .to_string(),
        };
        
        self.auth_tracker.add_attempt(attempt);
    }
    pub fn get_log_level(&self) -> SecurityLogLevel {
        self.log_level
    }

    pub fn get_auth_attempts(&self) -> Vec<AuthAttempt> {
        self.auth_tracker.attempts.lock().unwrap().clone()
    }

    fn check_suspicious_activity(&self, req: &ServiceRequest) -> bool {
        let conn_info = req.connection_info();
        let ip = conn_info.peer_addr().unwrap_or("unknown");
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        
        let mut attempts = self.activity_tracker.failed_attempts.lock().unwrap();
        let ip_attempts = attempts.entry(ip.to_string()).or_default();
        
        let cutoff = timestamp - self.activity_tracker.window_secs;
        ip_attempts.retain(|&t| t > cutoff);
        ip_attempts.push(timestamp);
        
        ip_attempts.len() >= self.activity_tracker.threshold
    }

    fn determine_severity(&self, success: bool, is_suspicious: bool) -> AuditEventType {
        match (success, is_suspicious) {
            (false, true) => AuditEventType::SuspiciousActivity,
            (false, false) => AuditEventType::Authentication,
            (true, _) => AuditEventType::Authorization,
        }
    }

    pub fn log_security_event(&mut self, req: &ServiceRequest, event_type: AuditEventType, success: bool) -> std::io::Result<()> {
        let conn_info = req.connection_info();
        let ip = conn_info.peer_addr().unwrap_or("unknown").to_string();
        let user_agent = req.headers()
            .get("User-Agent")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("unknown")
            .to_string();

        if matches!(event_type, AuditEventType::Authentication) {
            self.track_auth_attempt(req, success);
        }

        let is_suspicious = !success && self.check_suspicious_activity(req);

        let audit_event = AuditEvent {
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            event_type: self.determine_severity(success, is_suspicious),
            severity: if is_suspicious { AuditSeverity::Critical } 
                     else if !success { AuditSeverity::Warning }
                     else { AuditSeverity::Info },
            details: format!("Access attempt: {}", if success { "successful" } else { "failed" }),
            source_ip: ip,
            user_agent: Some(user_agent),
        };

        self.audit_logger.log_event(audit_event)
    }
}