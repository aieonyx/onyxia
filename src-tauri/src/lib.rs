// Copyright (c) 2026 Edison Lepiten / AIEONYX
// SPDX-License-Identifier: Apache-2.0

mod browser;
mod commands;

use browser::tab_manager::TabManager;
use std::sync::{Arc, Mutex};

pub fn run() {
    env_logger::init();

    let tab_manager = Arc::new(Mutex::new(TabManager::new()));

    tauri::Builder::default()
        .manage(tab_manager)
        .invoke_handler(tauri::generate_handler![
            commands::navigation::navigate,
            commands::navigation::go_back,
            commands::navigation::go_forward,
            commands::navigation::new_tab,
            commands::navigation::close_tab,
            commands::navigation::switch_tab,
        ])
        .run(tauri::generate_context!())
        .expect("Onyxia failed to start");
}
