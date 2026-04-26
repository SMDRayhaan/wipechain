use serde::Serialize;
use serde::Deserialize;
use crate::service::services::models::{ServiceDisk, WipePreview};

#[derive(Serialize)]
pub struct Disk {
    pub id: String,
    pub name: String,
    pub encrypted: bool,
}

#[derive(Serialize)]
pub struct WipePreviewResponse {
    pub volume_id: String,
    pub allowed: bool,
    pub reason: Option<String>,
}

#[derive(Deserialize)]
pub struct WipeRequest {
    pub dry_run: Option<bool>,
}

#[derive(Serialize)]
pub struct WipeResponse {
    pub volume_id: String,
    pub output: String,
}

impl From<ServiceDisk> for Disk {
    fn from(value: ServiceDisk) -> Self {
        Self {
            id: value.id,
            name: value.name,
            encrypted: value.encrypted,
        }
    }
}

impl From<WipePreview> for WipePreviewResponse {
    fn from(value: WipePreview) -> Self {
        Self {
            volume_id: value.volume_id,
            allowed: value.allowed,
            reason: value.reason,
        }
    }
}