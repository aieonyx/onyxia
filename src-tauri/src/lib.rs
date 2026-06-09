// Copyright (c) 2026 Edison Lepiten / AIEONYX
// SPDX-License-Identifier: Apache-2.0

mod browser;
mod commands;

use browser::arpi_state::PageState;
use browser::awp_handler::handle_awp_request;
use browser::header_injection::{axon_client_value, AXON_CLIENT_HEADER, should_inject_header};
use browser::tab_manager::TabManager;
use commands::page_state::CurrentPageState;
use std::sync::{Arc, Mutex};
use tauri::{Manager, WebviewWindowBuilder, WebviewUrl};

const CHROME_H: f64 = 108.0;
const WIN_W: f64 = 1280.0;
const WIN_H: f64 = 800.0;

pub fn run() {
    env_logger::init();

    let tab_manager = Arc::new(Mutex::new(TabManager::new()));
    let page_state: CurrentPageState = Arc::new(Mutex::new(PageState::blank()));
    let header_value = axon_client_value();

    log::info!("AXON-Client header: {}", header_value);

    tauri::Builder::default()
        .manage(tab_manager)
        .manage(page_state)
        .register_uri_scheme_protocol("awp", |_app, request| {
            handle_awp_request(request)
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
        ])
        .setup(move |app| {
            let header_val = header_value.clone();

            // Chrome window — fixed height, never navigates, has IPC
            let chrome = WebviewWindowBuilder::new(
                app,
                "chrome",
                WebviewUrl::App("index.html".into()),
            )
            .title("Onyxia")
            .inner_size(WIN_W, CHROME_H)
            .min_inner_size(800.0, CHROME_H)
            .resizable(false)
            .decorations(true)
            .position(0.0, 0.0)
            .build()?;

            // Content window — below chrome, navigates freely
            let content = WebviewWindowBuilder::new(
                app,
                "content",
                WebviewUrl::External("about:blank".parse().unwrap()),
            )
            .title("")
            .inner_size(WIN_W, WIN_H - CHROME_H)
            .min_inner_size(800.0, 400.0)
            .resizable(true)
            .decorations(false)
            .position(0.0, CHROME_H)
            .build()?;

            // Header injection on content window
            #[cfg(target_os = "linux")]
            {
                use webkit2gtk::{URIRequestExt, WebResourceExt, WebViewExt};
                let hv = header_val.clone();
                content.with_webview(move |wv| {
                    let inner = wv.inner();
                    let h = hv.clone();
                    inner.connect_resource_load_started(
                        move |_view,
                              resource: &webkit2gtk::WebResource,
                              request: &webkit2gtk::URIRequest| {
                            if let Some(url) = resource.uri() {
                                if should_inject_header(url.as_str()) {
                                    if let Some(headers) = request.http_headers() {
                                        headers.append(AXON_CLIENT_HEADER, h.as_str());
                                    }
                                }
                            }
                        },
                    );
                })?;
            }

            // Sync content window position/size when chrome moves or resizes
            let content_clone = content.clone();
            chrome.on_window_event(move |event| {
                match event {
                    tauri::WindowEvent::Moved(pos) => {
                        let _ = content_clone.set_position(
                            tauri::PhysicalPosition::new(pos.x, pos.y + CHROME_H as i32)
                        );
                    }
                    tauri::WindowEvent::Resized(size) => {
                        let _ = content_clone.set_size(
                            tauri::PhysicalSize::new(size.width, size.height)
                        );
                    }
                    tauri::WindowEvent::CloseRequested { .. } => {
                        let _ = content_clone.close();
                    }
                    _ => {}
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Onyxia failed to start");
}
