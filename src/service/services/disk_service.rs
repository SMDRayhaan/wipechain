use crate::service::dto::disk::Disk;
use crate::os::windows::volume::get_volumes;
use crate::os::windows::bitlocker::get_encryption_status;
use crate::os::windows::executor::run_command;

// =========================
// GET DISKS
// =========================
pub fn get_system_disks() -> Result<Vec<Disk>, String> {
    let volumes = get_volumes()?;
    let encryption = get_encryption_status()?;

    let result = volumes
        .into_iter()
        .map(|v| {
            let enc = encryption.iter().find(|e| {
                e.drive_letter.as_deref() == Some(&v.device_id)
            });

            Disk {
                id: v.device_id.clone(),
                name: v.name
                    .filter(|n| !n.is_empty())
                    .unwrap_or(format!("Volume {}", v.device_id)),
                encrypted: enc
                    .and_then(|e| e.protection_status)
                    .map(|s| s == 1)
                    .unwrap_or(false),
            }
        })
        .collect();

    Ok(result)
}

// =========================
// VALIDATION (PREVIEW ONLY)
// =========================
pub fn validate_volume_for_wipe(volume_id: &str) -> Result<(), String> {
    let volumes = get_volumes()?; // WMI

    let volume = volumes
        .into_iter()
        .find(|v| v.device_id == volume_id)
        .ok_or("Volume not found")?;

    if volume.device_id == "C:" {
        return Err("Refusing to wipe system volume (C:)".into());
    }

    Ok(())
}

// =========================
// ENCRYPTION CHECK (PREVIEW ONLY)
// =========================
pub fn is_volume_encrypted(volume_id: &str) -> Result<bool, String> {
    let encryption = get_encryption_status()?; // WMI

    let enc = encryption.iter().find(|e| {
        e.drive_letter.as_deref() == Some(volume_id)
    });

    Ok(enc
        .and_then(|e| e.protection_status)
        .map(|s| s == 1)
        .unwrap_or(false))
}

// =========================
// EXECUTION (NO WMI)
// =========================
pub fn wipe_volume(method: &str, volume_id: &str, dry_run: bool) -> Result<String, String> {

    if volume_id == "C:" {
        return Err("Refusing to wipe system volume (C:)".into());
    }

    let command = match method {
        "CRYPTO_ERASE" => format!("manage-bde -off {}", volume_id),
        "OVERWRITE" => format!("cipher /w:{}", volume_id),
        _ => return Err("Invalid wipe method".into()),
    };

    if dry_run {
        return Ok(format!("DRY RUN ({}): {}", method, command));
    }

    run_command("cmd", &["/C", &command])
}