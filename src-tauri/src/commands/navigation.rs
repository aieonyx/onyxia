// Copyright (c) 2026 Edison Lepiten / AIEONYX
// SPDX-License-Identifier: Apache-2.0

use crate::browser::tab_manager::{Tab, TabManager};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager, State};

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

    let active_id = {
        let mgr = tab_state.lock().map_err(|e: std::sync::PoisonError<_>| e.to_string())?;
        mgr.active_id()
    };

    if let Some(content) = app.get_webview_window("content") {
        content.navigate(
            url.parse().map_err(|e: url::ParseError| e.to_string())?
        ).map_err(|e: tauri::Error| e.to_string())?;
    }

    {
        let mut mgr = tab_state.lock().map_err(|e: std::sync::PoisonError<_>| e.to_string())?;
        mgr.update_tab_url(active_id, url.clone(), url.clone());
    }
    Ok(())
}

#[tauri::command]
pub async fn go_back(app: AppHandle) -> Result<(), String> {
    if let Some(content) = app.get_webview_window("content") {
        content.eval("window.history.back()").map_err(|e: tauri::Error| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn go_forward(app: AppHandle) -> Result<(), String> {
    if let Some(content) = app.get_webview_window("content") {
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
pub async fn switch_tab(id: u32, tab_state: State<'_, TabState>) -> Result<(), String> {
    let mut mgr = tab_state.lock().map_err(|e: std::sync::PoisonError<_>| e.to_string())?;
    mgr.switch_tab(id)
}
