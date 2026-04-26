pub struct ServiceDisk {
    pub id: String,
    pub name: String,
    pub encrypted: bool,
}

pub struct WipePreview {
    pub volume_id: String,
    pub command: String,
    pub allowed: bool,
    pub reason: Option<String>,
}