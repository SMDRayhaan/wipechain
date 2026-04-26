use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::os::windows::volume::get_volumes;
use crate::os::windows::bitlocker::get_encryption_status;

pub struct VolumeData {
    pub id: String,
    pub name: String,
    pub encrypted: bool,
}

struct Cache {
    data: Vec<VolumeData>,
    last_updated: Instant,
}

static mut CACHE: Option<Arc<Mutex<Cache>>> = None;

fn get_cache() -> Arc<Mutex<Cache>> {
    unsafe {
        if CACHE.is_none() {
            CACHE = Some(Arc::new(Mutex::new(Cache {
                data: Vec::new(),
                last_updated: Instant::now() - Duration::from_secs(10),
            })));
        }
        CACHE.clone().unwrap()
    }
}

pub fn fetch_volumes() -> Result<Vec<VolumeData>, String> {
    let cache = get_cache();
    let mut cache_lock = cache.lock().unwrap();

    // 🔥 cache valid for 5 seconds
    if cache_lock.last_updated.elapsed() < Duration::from_secs(5) {
        println!("✅ CACHE HIT → returning cached data");
        return Ok(cache_lock.data.clone());
    }
    
    println!("⚡ CACHE MISS → fetching from WMI");

    let volumes = get_volumes()?;
    let encryption = get_encryption_status()?;

    let result: Vec<VolumeData> = volumes
        .into_iter()
        .map(|v| {
            let enc = encryption.iter().find(|e| {
                e.drive_letter.as_deref() == Some(&v.device_id)
            });

            VolumeData {
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

    cache_lock.data = result.clone();
    cache_lock.last_updated = Instant::now();

    Ok(result)
}