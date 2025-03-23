use log::{info, warn, error};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::{OpenOptions, File};
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditEvent {
    timestamp: u64,
    event_type: AuditEventType,
    severity: AuditSeverity,
    details: String,
    source_ip: String,
    user_agent: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AuditEventType {
    Authentication,
    Authorization,
    SuspiciousActivity,
    SystemAccess,
    Configuration,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AuditSeverity {
    Info,
    Warning,
    Critical,
}

pub struct AuditLogger {
    log_file: File,
}

impl AuditLogger {
    pub fn new(log_path: PathBuf) -> std::io::Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)?;
            
        Ok(AuditLogger { log_file: file })
    }

    pub fn log_event(&mut self, event: AuditEvent) -> std::io::Result<()> {
        let event_json = serde_json::to_string(&event)?;
        writeln!(self.log_file, "{}", event_json)?;
        
        match event.severity {
            AuditSeverity::Info => info!("Audit: {}", event_json),
            AuditSeverity::Warning => warn!("Audit: {}", event_json),
            AuditSeverity::Critical => error!("Audit: {}", event_json),
        }
        
        Ok(())
    }
}
