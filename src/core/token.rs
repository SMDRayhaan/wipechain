use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
pub struct WipeToken {
    pub volume_id: String,
    pub method: String,
    pub expires_at: u64,
}

const SECRET: &str = "super_secret_key";

pub fn generate_token(volume_id: &str, method: &str) -> String {
    let expires_at = current_time() + 60;

    let payload = WipeToken {
        volume_id: volume_id.to_string(),
        method: method.to_string(),
        expires_at,
    };

    let json = serde_json::to_string(&payload).unwrap();
    let signature = format!("{}{}", json, SECRET);

    base64::encode(format!("{}::{}", json, signature))
}

pub fn verify_token(token: &str, expected_volume: &str) -> Result<WipeToken, String> {
    let decoded = base64::decode(token)
        .map_err(|_| "Invalid token encoding")?;

    let decoded_str = String::from_utf8(decoded)
        .map_err(|_| "Invalid UTF-8")?;

    let parts: Vec<&str> = decoded_str.split("::").collect();

    if parts.len() != 2 {
        return Err("Invalid token format".into());
    }

    let json = parts[0];
    let signature = parts[1];

    let expected_signature = format!("{}{}", json, SECRET);

    if signature != expected_signature {
        return Err("Invalid signature".into());
    }

    let token: WipeToken =
        serde_json::from_str(json).map_err(|_| "Invalid payload")?;

    if token.volume_id != expected_volume {
        return Err("Token volume mismatch".into());
    }

    if current_time() > token.expires_at {
        return Err("Token expired".into());
    }

    Ok(token)
}

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}