use serde::Deserialize;
use wmi::{COMLibrary, WMIConnection};

#[derive(Deserialize, Debug)]
pub struct Volume {
    #[serde(rename = "DeviceID")]
    pub device_id: String,

    #[serde(rename = "VolumeName")]
    pub name: Option<String>,
}

pub fn get_volumes() -> Result<Vec<Volume>, String> {
    let com_con = COMLibrary::new().map_err(|e| e.to_string())?;
    let wmi_con = WMIConnection::new(com_con).map_err(|e| e.to_string())?;

    let results: Vec<Volume> = wmi_con
        .raw_query("SELECT DeviceID, VolumeName FROM Win32_LogicalDisk")
        .map_err(|e| e.to_string())?;

    Ok(results)
}