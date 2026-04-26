use serde::Deserialize;
use wmi::{COMLibrary, WMIConnection};

#[derive(Deserialize, Debug)]
pub struct Disk {
    #[serde(rename = "DeviceID")]
    pub device_id: String,

    #[serde(rename = "Model")]
    pub model: Option<String>,
}

pub fn get_disk_drives() -> Result<Vec<Disk>, String> {
    let com_con = COMLibrary::new().map_err(|e| e.to_string())?;
    let wmi_con = WMIConnection::new(com_con).map_err(|e| e.to_string())?;

    let results: Vec<Disk> = wmi_con
        .raw_query("SELECT DeviceID, Model FROM Win32_DiskDrive")
        .map_err(|e| e.to_string())?;

    Ok(results)
}