// Copyright (c) 2026 Edison Lepiten / AIEONYX
// SPDX-License-Identifier: Apache-2.0
//
// C5-A: Page state IPC commands.
// INVARIANT: frontend never computes protocol or ARPi state.

use crate::browser::arpi_state::PageState;
use std::sync::{Arc, Mutex};
use tauri::State;

pub type CurrentPageState = Arc<Mutex<PageState>>;

#[tauri::command]
pub fn get_page_state(
    page_state: State<'_, CurrentPageState>,
) -> Result<PageState, String> {
    let state = page_state.lock().map_err(|e| e.to_string())?;
    Ok(state.clone())
}

#[tauri::command]
pub fn update_page_url(
    url: String,
    page_state: State<'_, CurrentPageState>,
) -> Result<PageState, String> {
    let new_state = PageState::from_url(&url);
    let mut state = page_state.lock().map_err(|e| e.to_string())?;
    *state = new_state.clone();
    Ok(new_state)
}
