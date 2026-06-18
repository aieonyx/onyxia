// Copyright (c) 2026 Edison Lepiten / AIEONYX
// SPDX-License-Identifier: Apache-2.0

mod browser;
mod ca;
mod commands;

use browser::arpi_state::PageState;
use browser::awp_handler::handle_awp_request;
use browser::header_injection::{axon_client_value, AXON_CLIENT_HEADER, should_inject_header};
use browser::sts::{is_tracker, is_mixed_content};
use browser::tab_manager::TabManager;
use commands::page_state::CurrentPageState;
use std::sync::{Arc, Mutex};
use tauri::{LogicalPosition, LogicalSize, Emitter};
use tauri::webview::WebviewBuilder;
use tauri::window::WindowBuilder;

const CHROME_H: f64 = 160.0;
const WIN_W:    f64 = 1280.0;
const WIN_H:    f64 = 800.0;

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
            let header_val = header_value.clone();

            let window = WindowBuilder::new(app, "main")
                .inner_size(WIN_W, WIN_H)
                .min_inner_size(800.0, 400.0)
                .decorations(false)
                .build()?;

            let _chrome = window.add_child(
                WebviewBuilder::new(
                    "chrome",
                    tauri::WebviewUrl::App("index.html".into()),
                ),
                LogicalPosition::new(0.0, 0.0),
                LogicalSize::new(WIN_W, CHROME_H),
            )?;

            let content = window.add_child(
                WebviewBuilder::new(
                    "content",
                    tauri::WebviewUrl::External("about:blank".parse().unwrap()),
                )
                .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36"),
                LogicalPosition::new(0.0, CHROME_H),
                LogicalSize::new(WIN_W, WIN_H - CHROME_H),
            )?;

            // C14: install AIEONYX Root CA into WebKitGTK TLS database
            #[cfg(target_os = "linux")]
            {
                if let Err(e) = ca::install_aieonyx_ca(&content) {
                    log::warn!("C14: CA installation warning: {}", e);
                }
            }

            // GtkBox surgery: repack children with correct expand flags
            #[cfg(target_os = "linux")]
            {
                use gtk::prelude::{BoxExt, ContainerExt, WidgetExt};
                if let Ok(vbox) = window.default_vbox() {
                    let children = vbox.children();
                    if children.len() >= 2 {
                        let chrome_widget = children[0].clone();
                        let content_widget = children[1].clone();
                        vbox.remove(&chrome_widget);
                        vbox.remove(&content_widget);
                        chrome_widget.set_size_request(-1, CHROME_H as i32);
                        vbox.pack_start(&chrome_widget, false, false, 0);
                        vbox.pack_start(&content_widget, true, true, 0);
                        vbox.show_all();
                    }
                }
            }


            // Track content webview URL changes and sync to tab state
            #[cfg(target_os = "linux")]
            {
                use webkit2gtk::WebViewExt;
                let app_handle = app.handle().clone();
                content.with_webview(move |wv| {
                    let inner = wv.inner();
                    let app2 = app_handle.clone();
                    inner.connect_uri_notify(move |view| {
                        if let Some(uri) = view.uri() {
                            let uri_str = uri.to_string();
                            let blank = uri_str == "about:blank";
                            let empty = uri_str.is_empty();
                            if !blank && !empty {
                                let _ = app2.emit("url-changed", uri_str);
                            }
                        }
                    });
                    let app3t = app_handle.clone();
                    inner.connect_title_notify(move |view| {
                        if let Some(title) = view.title() {
                            let t = title.to_string();
                            let empty = t.is_empty();
                            if !empty {
                                let _ = app3t.emit("title-changed", t);
                            }
                        }
                    });
                })?;
            }

            // Header injection
            #[cfg(target_os = "linux")]
            {
                use webkit2gtk::{URIRequestExt, WebResourceExt, WebViewExt};
                let hv = header_val.clone();
                let app3 = app.handle().clone();
                content.with_webview(move |wv| {
                    let inner = wv.inner();
                    let h = hv.clone();
                    let app3 = app3.clone();
                    inner.connect_resource_load_started(
                        move |view,
                              resource: &webkit2gtk::WebResource,
                              request: &webkit2gtk::URIRequest| {
                            if let Some(url) = resource.uri() {
                                let url_str = url.as_str();
                                // C3: AXON-Client header injection
                                if should_inject_header(url_str) {
                                    if let Some(headers) = request.http_headers() {
                                        headers.append(AXON_CLIENT_HEADER, h.as_str());
                                    }
                                }
                                // C7-A: STS tracker detection
                                if let Some(threat) = is_tracker(url_str) {
                                    log::warn!("STS: tracker blocked: {} on {}", threat.domain, url_str);
                                    let _ = app3.emit("threat-detected", &threat);
                                }
                                // C7-A: STS mixed content detection
                                if let Some(page_url) = view.uri() {
                                    if let Some(threat) = is_mixed_content(page_url.as_str(), url_str) {
                                        log::warn!("STS: mixed content: {}", url_str);
                                        let _ = app3.emit("threat-detected", &threat);
                                    }
                                }
                            }
                        },
                    );
                })?;
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Onyxia failed to start");
}
