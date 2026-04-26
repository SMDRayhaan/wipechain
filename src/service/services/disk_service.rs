use crate::service::dto::disk::Disk;
use crate::os::windows::volume::get_volumes;
use crate::os::windows::bitlocker::get_encryption_status;
use crate::os::windows::executor::run_command;

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

pub fn validate_volume_for_wipe(volume_id: &str) -> Result<(), String> {
    let volumes = get_volumes()?;

    let volume = volumes
        .into_iter()
        .find(|v| v.device_id == volume_id)
        .ok_or("Volume not found")?;

    if volume.device_id == "C:" {
        return Err("Refusing to wipe system volume (C:)".into());
    }

    Ok(())
}

pub fn wipe_volume(volume_id: &str, dry_run: bool) -> Result<String, String> {
    validate_volume_for_wipe(volume_id)?;

    // 🔍 Check encryption
    let encryption = get_encryption_status()?;

    let is_encrypted = encryption.iter().any(|e| {
        e.drive_letter.as_deref() == Some(volume_id)
            && e.protection_status == Some(1)
    });

    // 🧠 Strategy decision
    if is_encrypted {
        let command = format!("manage-bde -off {}", volume_id);

        if dry_run {
            return Ok(format!("DRY RUN (CRYPTO ERASE): {}", command));
        }

        return run_command("cmd", &["/C", &command]);
    }

    // fallback → overwrite
    let command = format!("cipher /w:{}", volume_id);

    if dry_run {
        return Ok(format!("DRY RUN (OVERWRITE): {}", command));
    }

    run_command("cmd", &["/C", &command])
}