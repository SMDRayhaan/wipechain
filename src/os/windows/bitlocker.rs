use serde::Deserialize;
use wmi::{COMLibrary, WMIConnection};

#[derive(Deserialize, Debug)]
pub struct VolumeEncryption {
    #[serde(rename = "DriveLetter")]
    pub drive_letter: Option<String>,

    #[serde(rename = "ProtectionStatus")]
    pub protection_status: Option<u32>,
}

pub fn get_encryption_status() -> Result<Vec<VolumeEncryption>, String> {
    let com_con = COMLibrary::new().map_err(|e| e.to_string())?;
    let wmi_con = WMIConnection::with_namespace_path(
        "ROOT\\CIMV2\\Security\\MicrosoftVolumeEncryption",
        com_con,
    )
    .map_err(|e| e.to_string())?;

    let results: Vec<VolumeEncryption> = wmi_con
        .raw_query(
            "SELECT DriveLetter, ProtectionStatus FROM Win32_EncryptableVolume",
        )
        .map_err(|e| e.to_string())?;

    Ok(results)
}