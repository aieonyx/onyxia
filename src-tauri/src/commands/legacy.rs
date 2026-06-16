// Copyright (c) 2026 Edison Lepiten / AIEONYX
// SPDX-License-Identifier: Apache-2.0
//
// C10: Digital Legacy — Data Testament and Inactivity Heartbeat
// INVARIANT: testament stored in EdisonDB Critical tier only.
// INVARIANT: frontend never computes legacy state.

use serde::{Deserialize, Serialize};

const EDISONDB_URL: &str = "http://localhost:7777";
const OWNER_ID: &str = "onyxia";
const PASSWORD: &str = "sovereign";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Testament {
    pub inactivity_days: u32,
    pub critical_action: String,   // "delete" | "preserve"
    pub personal_action: String,   // "delete" | "transfer"
    pub noise_action: String,      // "purge" | "preserve"
    pub legacy_holder_name: String,
    pub legacy_holder_contact: String,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heartbeat {
    pub last_active: u64,
    pub testament_active: bool,
}

fn unix_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

async fn db_delete(client: &reqwest::Client, id: &str) {
    let _ = client
        .delete(format!("{}/api/delete/{}", EDISONDB_URL, id))
        .header("X-Owner-ID", OWNER_ID)
        .header("X-Password", PASSWORD)
        .send()
        .await;
}

async fn db_write(client: &reqwest::Client, id: &str, tier: &str, payload: &str) -> Result<(), String> {
    db_delete(client, id).await;
    client
        .post(format!("{}/api/write", EDISONDB_URL))
        .header("X-Owner-ID", OWNER_ID)
        .header("X-Password", PASSWORD)
        .json(&serde_json::json!({
            "id": id,
            "tier": tier,
            "payload": payload
        }))
        .send()
        .await
        .map_err(|e| format!("EdisonDB unreachable: {}", e))?;
    Ok(())
}

#[tauri::command]
pub async fn save_testament(testament: Testament) -> Result<(), String> {
    let client = reqwest::Client::new();
    let payload = serde_json::to_string(&testament).map_err(|e| e.to_string())?;
    db_write(&client, "legacy:testament", "Critical", &payload).await?;
    log::info!("Testament saved: {} day trigger, holder: {}", testament.inactivity_days, testament.legacy_holder_name);
    Ok(())
}

#[tauri::command]
pub async fn load_testament() -> Result<Option<Testament>, String> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/api/read/legacy:testament", EDISONDB_URL))
        .header("X-Owner-ID", OWNER_ID)
        .header("X-Password", PASSWORD)
        .send()
        .await
        .map_err(|e| format!("EdisonDB unreachable: {}", e))?;
    if resp.status().as_u16() == 404 { return Ok(None); }
    let record: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
    let payload = record["payload"].as_str().ok_or("missing payload")?;
    let t: Testament = serde_json::from_str(payload).map_err(|e| e.to_string())?;
    Ok(Some(t))
}

#[tauri::command]
pub async fn ping_heartbeat() -> Result<Heartbeat, String> {
    let client = reqwest::Client::new();
    let now = unix_now();
    let testament = load_testament().await.unwrap_or(None);
    let hb = Heartbeat {
        last_active: now,
        testament_active: testament.is_some(),
    };
    let payload = serde_json::to_string(&hb).map_err(|e| e.to_string())?;
    db_write(&client, "legacy:heartbeat", "Personal", &payload).await?;
    log::info!("Heartbeat: last_active={}, testament={}", now, hb.testament_active);
    Ok(hb)
}

#[tauri::command]
pub async fn get_legacy_status() -> Result<serde_json::Value, String> {
    let client = reqwest::Client::new();
    // Read heartbeat
    let hb_resp = client
        .get(format!("{}/api/read/legacy:heartbeat", EDISONDB_URL))
        .header("X-Owner-ID", OWNER_ID)
        .header("X-Password", PASSWORD)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let testament = load_testament().await.unwrap_or(None);
    let now = unix_now();

    let (last_active, days_inactive) = if hb_resp.status().as_u16() == 200 {
        let record: serde_json::Value = hb_resp.json().await.unwrap_or_default();
        let payload = record["payload"].as_str().unwrap_or("{}");
        let hb: serde_json::Value = serde_json::from_str(payload).unwrap_or_default();
        let la = hb["last_active"].as_u64().unwrap_or(now);
        let inactive = (now - la) / 86400;
        (la, inactive)
    } else {
        (now, 0)
    };

    let trigger_days = testament.as_ref().map(|t| t.inactivity_days).unwrap_or(180);
    let status = if testament.is_none() {
        "no_testament"
    } else if days_inactive >= trigger_days as u64 {
        "triggered"
    } else if days_inactive >= (trigger_days / 2) as u64 {
        "warning"
    } else {
        "active"
    };

    Ok(serde_json::json!({
        "status": status,
        "days_inactive": days_inactive,
        "trigger_days": trigger_days,
        "testament_active": testament.is_some(),
        "last_active": last_active,
        "holder": testament.as_ref().map(|t| t.legacy_holder_name.clone()).unwrap_or_default()
    }))
}
