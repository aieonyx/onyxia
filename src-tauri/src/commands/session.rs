// Copyright (c) 2026 Edison Lepiten / AIEONYX
// SPDX-License-Identifier: Apache-2.0
//
// C8: EdisonDB session persistence
// Saves and restores tab sessions via EdisonDB REST API.

use serde::{Deserialize, Serialize};

const EDISONDB_URL: &str = "http://localhost:7777";
const OWNER_ID: &str = "onyxia";
const PASSWORD: &str = "sovereign";
const SESSION_ID: &str = "session:tabs";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabSession {
    pub tabs: Vec<SessionTab>,
    pub active: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionTab {
    pub id: u32,
    pub url: String,
    pub title: String,
}

#[tauri::command]
pub async fn save_session(
    tabs: Vec<SessionTab>,
    active_id: u32,
) -> Result<(), String> {
    log::info!("save_session called: {} tabs, active={}", tabs.len(), active_id);
    let session = TabSession { tabs, active: active_id };
    let payload = serde_json::to_string(&session)
        .map_err(|e| e.to_string())?;

    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "id": SESSION_ID,
        "tier": "Personal",
        "payload": payload
    });

    // Delete existing record first (EdisonDB has no upsert)
    let _ = client
        .delete(format!("{}/api/delete/{}", EDISONDB_URL, SESSION_ID))
        .header("X-Owner-ID", OWNER_ID)
        .header("X-Password", PASSWORD)
        .send()
        .await;

    // Write new record
    let resp = client
        .post(format!("{}/api/write", EDISONDB_URL))
        .header("X-Owner-ID", OWNER_ID)
        .header("X-Password", PASSWORD)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("EdisonDB unreachable: {}", e))?;

    log::info!("save_session write: {} -> {}", session.tabs.len(), resp.status());
    Ok(())
}

#[tauri::command]
pub async fn load_session() -> Result<Option<TabSession>, String> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/api/read/{}", EDISONDB_URL, SESSION_ID))
        .header("X-Owner-ID", OWNER_ID)
        .header("X-Password", PASSWORD)
        .send()
        .await
        .map_err(|e| format!("EdisonDB unreachable: {}", e))?;

    if resp.status() == 404 {
        return Ok(None);
    }

    let record: serde_json::Value = resp.json().await
        .map_err(|e| e.to_string())?;

    let payload = record["payload"].as_str()
        .ok_or("missing payload")?;

    let session: TabSession = serde_json::from_str(payload)
        .map_err(|e| e.to_string())?;

    Ok(Some(session))
}
