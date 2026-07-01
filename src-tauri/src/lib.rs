// Copyright (c) 2026 Edison Lepiten / AIEONYX
// SPDX-License-Identifier: Apache-2.0

mod browser;
mod ca;
mod commands;

use browser::arpi_state::PageState;
use browser::awp_handler::handle_awp_request;
use browser::haniel_handler::{handle_haniel_request, HanielState};
use browser::header_injection::axon_client_value;
use browser::tab_manager::TabManager;
use commands::page_state::CurrentPageState;
use std::sync::{Arc, Mutex};
use tauri::{LogicalPosition, LogicalSize};
use tauri::webview::WebviewBuilder;
use tauri::window::WindowBuilder;

const CHROME_H: f64 = 160.0;
const WIN_W:    f64 = 1280.0;
const WIN_H:    f64 = 800.0;

pub fn run() {
    env_logger::init();

    let tab_manager  = Arc::new(Mutex::new(TabManager::new()));
    let page_state: CurrentPageState = Arc::new(Mutex::new(PageState::blank()));
    let header_value = axon_client_value();
    let haniel_state = Arc::new(HanielState::new(
        WIN_W as u32,
        (WIN_H - CHROME_H) as u32,
        header_value.clone(),
    ));

    log::info!("AXON-Client header: {}", header_value);

    tauri::Builder::default()
        .manage(tab_manager)
        .manage(page_state)
        .manage(haniel_state.clone())
        .register_uri_scheme_protocol("awp", |_app, request| {
            handle_awp_request(request)
        })
        .register_uri_scheme_protocol("haniel", move |ctx, request| {
            handle_haniel_request(&haniel_state, ctx.app_handle(), request)
        })
        .invoke_handler(tauri::generate_handler![
            commands::navigation::navigate,
            commands::navigation::go_back,
            commands::navigation::go_forward,
            commands::navigation::new_tab,
            commands::navigation::close_tab,
            commands::navigation::switch_tab,
            commands::page_state::get_page_state,
            commands::page_state::update_page_url,
            commands::window_controls::minimize_window,
            commands::window_controls::maximize_window,
            commands::window_controls::close_window,
            commands::window_controls::start_drag,
            commands::session::save_session,
            commands::session::load_session,
            commands::legacy::save_testament,
            commands::legacy::load_testament,
            commands::legacy::ping_heartbeat,
            commands::legacy::get_legacy_status,
        ])
        .setup(move |app| {
            let window = WindowBuilder::new(app, "main")
                .inner_size(WIN_W, WIN_H)
                .min_inner_size(800.0, 400.0)
                .decorations(false)
                .build()?;

            let _chrome = window.add_child(
                WebviewBuilder::new("chrome", tauri::WebviewUrl::App("index.html".into())),
                LogicalPosition::new(0.0, 0.0),
                LogicalSize::new(WIN_W, CHROME_H),
            )?;

            let _content = window.add_child(
                WebviewBuilder::new(
                    "content",
                    tauri::WebviewUrl::External("about:blank".parse().unwrap()),
                )
                .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36"),
                LogicalPosition::new(0.0, CHROME_H),
                LogicalSize::new(WIN_W, WIN_H - CHROME_H),
            )?;

            if let Err(e) = ca::install_aieonyx_ca() {
                log::warn!("CA install: {}", e);
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Onyxia failed to start");
}
