// Copyright (c) 2026 Edison Lepiten / AIEONYX
// SPDX-License-Identifier: Apache-2.0

use crate::browser::tab_manager::{Tab, TabManager};
use crate::browser::ssv::verify_site;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, Manager, State};

type TabState = Arc<Mutex<TabManager>>;

#[tauri::command]
pub async fn navigate(
    url: String,
    tab_state: State<'_, TabState>,
    app: AppHandle,
) -> Result<(), String> {
    let url = url.trim().to_string();
    let url = if !url.starts_with("http://")
        && !url.starts_with("https://")
        && !url.starts_with("awp://")
    {
        format!("https://{}", url)
    } else {
        url
    };

    if !url.starts_with("http://")
        && !url.starts_with("https://")
        && !url.starts_with("awp://")
    {
        return Err("Unsupported scheme.".to_string());
    }

    // C7-B: SSV check before navigation
    if let Some(verdict) = verify_site(url.as_str()) {
        log::warn!("SSV blocked: {} -> {}", verdict.threat.kind, verdict.threat.domain);
        let _ = app.emit("ssv-blocked", &verdict);
        return Err(format!("SSV: {} blocked — {}", verdict.threat.kind, verdict.threat.domain));
    }

    if let Some(content) = app.get_webview("content") {
        content
            .navigate(url.parse().map_err(|e: url::ParseError| e.to_string())?)
            .map_err(|e: tauri::Error| e.to_string())?;
    } else {
        return Err("Content webview not found".to_string());
    }

    let active_id = {
        let mgr = tab_state.lock().map_err(|e: std::sync::PoisonError<_>| e.to_string())?;
        mgr.active_id()
    };
    {
        let mut mgr = tab_state.lock().map_err(|e: std::sync::PoisonError<_>| e.to_string())?;
        mgr.update_tab_url(active_id, url.clone(), url.clone());
    }
    Ok(())
}

#[tauri::command]
pub async fn go_back(app: AppHandle) -> Result<(), String> {
    if let Some(content) = app.get_webview("content") {
        content.eval("window.history.back()").map_err(|e: tauri::Error| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn go_forward(app: AppHandle) -> Result<(), String> {
    if let Some(content) = app.get_webview("content") {
        content.eval("window.history.forward()").map_err(|e: tauri::Error| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn new_tab(tab_state: State<'_, TabState>) -> Result<Tab, String> {
    let mut mgr = tab_state.lock().map_err(|e: std::sync::PoisonError<_>| e.to_string())?;
    let tab = mgr.new_tab().clone();
    Ok(tab)
}

#[tauri::command]
pub async fn close_tab(id: u32, tab_state: State<'_, TabState>) -> Result<(), String> {
    let mut mgr = tab_state.lock().map_err(|e: std::sync::PoisonError<_>| e.to_string())?;
    mgr.close_tab(id)
}

#[tauri::command]
pub async fn switch_tab(id: u32, tab_state: State<'_, TabState>, app: AppHandle) -> Result<(), String> {
    let url = {
        let mut mgr = tab_state.lock().map_err(|e: std::sync::PoisonError<_>| e.to_string())?;
        mgr.switch_tab(id)?;
        mgr.active_tab().map(|t| t.url.clone())
    };
    if let Some(url) = url {
        // C7-B: SSV check before navigation
    if let Some(verdict) = verify_site(url.as_str()) {
        log::warn!("SSV blocked: {} -> {}", verdict.threat.kind, verdict.threat.domain);
        let _ = app.emit("ssv-blocked", &verdict);
        return Err(format!("SSV: {} blocked — {}", verdict.threat.kind, verdict.threat.domain));
    }

    if let Some(content) = app.get_webview("content") {
            let target = if url == "about:blank" || url.is_empty() {
                "about:blank".to_string()
            } else {
                url
            };
            let _ = content.navigate(target.parse().unwrap());
        }
    }
    Ok(())
}
