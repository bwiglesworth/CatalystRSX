use serde::{Serialize, Deserialize};
use std::fs::{OpenOptions, File};
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditEvent {
    pub timestamp: u64,
    pub event_type: AuditEventType,
    pub severity: AuditSeverity,
    pub details: String,
    pub source_ip: String,
    pub user_agent: Option<String>,
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
    pub fn new(log_path: std::path::PathBuf) -> std::io::Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)?;
            
        Ok(AuditLogger { log_file: file })
    }

    pub fn log_event(&mut self, event: AuditEvent) -> std::io::Result<()> {
        let event_json = serde_json::to_string(&event)?;
        writeln!(self.log_file, "{}", event_json)
    }
}