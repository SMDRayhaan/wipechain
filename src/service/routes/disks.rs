use axum::{
    Json,
    http::StatusCode,
    extract::Path,
};
use serde::Deserialize;

use crate::service::dto::disk::Disk;
use crate::service::services::disk_service::{
    get_system_disks,
    validate_volume_for_wipe,
    is_volume_encrypted,
    wipe_volume,
};

use crate::core::token::{generate_token, verify_token};
use crate::core::logger::{log_action, create_log};

// =========================
// GET /disks
// =========================
pub async fn disks_handler() -> Result<Json<Vec<Disk>>, (StatusCode, String)> {
    let disks = get_system_disks()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(disks))
}

// =========================
// GET /preview/:id
// =========================
pub async fn preview_wipe_handler(
    Path(volume_id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {

    validate_volume_for_wipe(&volume_id)
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    let encrypted = is_volume_encrypted(&volume_id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let method = if encrypted {
        "CRYPTO_ERASE"
    } else {
        "OVERWRITE"
    };

    let token = generate_token(&volume_id, method);

    Ok(Json(serde_json::json!({
        "volume_id": volume_id,
        "encrypted": encrypted,
        "method": method,
        "token": token
    })))
}

// =========================
// POST /wipe/:id
// =========================
#[derive(Deserialize)]
pub struct WipeRequest {
    pub token: String,
    pub dry_run: Option<bool>,
}

pub async fn wipe_handler(
    Path(volume_id): Path<String>,
    Json(body): Json<WipeRequest>,
) -> Result<Json<String>, (StatusCode, String)> {

    let token = verify_token(&body.token, &volume_id)
        .map_err(|e| (StatusCode::UNAUTHORIZED, e))?;

    let dry_run = body.dry_run.unwrap_or(false);

    let result = wipe_volume(&token.method, &volume_id, dry_run);

    // 🔥 LOGGING
    let status = match &result {
        Ok(_) => "SUCCESS",
        Err(_) => "FAILED",
    };

    let log = create_log(
        &volume_id,
        &token.method,
        dry_run,
        status,
    );

    let _ = log_action(log); // ignore logging failure

    let output = result
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok(Json(output))
}