use log::{warn, info};
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use std::sync::Mutex;
use actix_web::dev::ServiceRequest;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthAttempt {
    timestamp: u64,
    ip: String,
    path: String,
    success: bool,
    user_agent: String,
}

pub struct AuthTracker {
    attempts: Mutex<Vec<AuthAttempt>>,
    max_entries: usize,
}

impl AuthTracker {
    pub fn new(max_entries: usize) -> Self {
        AuthTracker {
            attempts: Mutex::new(Vec::new()),
            max_entries,
        }
    }

    pub fn record_attempt(&self, attempt: AuthAttempt) {
        let mut attempts = self.attempts.lock().unwrap();
        attempts.push(attempt);
        if attempts.len() > self.max_entries {
            attempts.remove(0); // Remove oldest entry when limit reached
        }
    }
}

impl SecurityLogger {
    // Add auth tracking to existing implementation
    pub fn track_auth_attempt(&self, req: &ServiceRequest, success: bool) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        let conn_info = req.connection_info();
        let ip = conn_info.peer_addr().unwrap_or("unknown").to_string();
        let user_agent = req.headers()
            .get("User-Agent")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("unknown")
            .to_string();

        let attempt = AuthAttempt {
            timestamp,
            ip,
            path: req.path().to_string(),
            success,
            user_agent,
        };

        let tracker = AuthTracker::new(1000); // Keep last 1000 attempts
        tracker.record_attempt(attempt);
    }
}
#[derive(Default)]
pub struct ActivityTracker {
    failed_attempts: Mutex<HashMap<String, Vec<u64>>>,
    threshold: usize,
    window_secs: u64,
}

impl ActivityTracker {
    pub fn new(threshold: usize, window_secs: u64) -> Self {
        ActivityTracker {
            failed_attempts: Mutex::new(HashMap::new()),
            threshold,
            window_secs,
        }
    }

    pub fn track_attempt(&self, ip: &str, timestamp: u64) -> bool {
        let mut attempts = self.failed_attempts.lock().unwrap();
        let ip_attempts = attempts.entry(ip.to_string()).or_default();
        
        // Clean old attempts
        let cutoff = timestamp - self.window_secs;
        ip_attempts.retain(|&t| t > cutoff);
        
        // Add new attempt
        ip_attempts.push(timestamp);
        
        // Check if suspicious
        ip_attempts.len() >= self.threshold
    }
}

pub struct SecurityLogger {
    log_level: SecurityLogLevel,
}

#[derive(Clone, Copy)]
pub enum SecurityLogLevel {
    High,
    Medium,
    Low,
}

impl SecurityLogger {
    pub fn new(level: SecurityLogLevel) -> Self {
        SecurityLogger { 
            log_level: level 
        }
    }

    pub fn log_auth_attempt(&self, req: &ServiceRequest, success: bool) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        let conn_info = req.connection_info();
        let ip = conn_info.peer_addr().unwrap_or("unknown");
        
        match (success, self.log_level) {
            (true, SecurityLogLevel::High) => info!(
                "Authentication success - IP: {}, Timestamp: {}, Path: {}, Level: High", 
                ip, timestamp, req.path()
            ),
            (false, _) => warn!(
                "Authentication failure - IP: {}, Timestamp: {}, Path: {}", 
                ip, timestamp, req.path()
            ),
            _ => info!(
                "Authentication success - IP: {}, Timestamp: {}, Path: {}", 
                ip, timestamp, req.path()
            ),
        }
    }

    pub fn check_suspicious_activity(&self, req: &ServiceRequest, success: bool) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        let conn_info = req.connection_info();
        let ip = conn_info.peer_addr().unwrap_or("unknown");

        if !success {
            let tracker = ActivityTracker::new(5, 300); // 5 attempts within 5 minutes
            if tracker.track_attempt(ip, timestamp) {
                warn!(
                    "SUSPICIOUS ACTIVITY DETECTED - IP: {}, Multiple failed attempts, Path: {}", 
                    ip, req.path()
                );
            }
        }
    }
}