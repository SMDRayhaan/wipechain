use std::fs::{OpenOptions};
use std::io::Write;
use serde::Serialize;
use chrono::Utc;

#[derive(Serialize)]
pub struct AuditLog {
    pub volume_id: String,
    pub method: String,
    pub timestamp: i64,
    pub dry_run: bool,
    pub status: String,
}

pub fn log_action(log: AuditLog) -> Result<(), String> {
    let json = serde_json::to_string(&log)
        .map_err(|e| e.to_string())?;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logs.json")
        .map_err(|e| e.to_string())?;

    writeln!(file, "{}", json)
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn create_log(
    volume_id: &str,
    method: &str,
    dry_run: bool,
    status: &str,
) -> AuditLog {
    AuditLog {
        volume_id: volume_id.to_string(),
        method: method.to_string(),
        timestamp: Utc::now().timestamp(),
        dry_run,
        status: status.to_string(),
    }
}