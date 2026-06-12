// Copyright (c) 2026 Edison Lepiten / AIEONYX
// SPDX-License-Identifier: Apache-2.0

use tauri::{AppHandle, Manager};

#[tauri::command]
pub async fn minimize_window(app: AppHandle) -> Result<(), String> {
    if let Some(w) = app.get_window("main") {
        w.minimize().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn maximize_window(app: AppHandle) -> Result<(), String> {
    if let Some(w) = app.get_window("main") {
        let is_max = w.is_maximized().unwrap_or(true);
        if is_max {
            w.unmaximize().map_err(|e| e.to_string())?;
        } else {
            w.maximize().map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn close_window(app: AppHandle) -> Result<(), String> {
    if let Some(w) = app.get_window("main") {
        w.close().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn start_drag(app: AppHandle) -> Result<(), String> {
    if let Some(w) = app.get_window("main") {
        w.start_dragging().map_err(|e| e.to_string())?;
    }
    Ok(())
}
