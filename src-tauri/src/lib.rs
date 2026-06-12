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
use tauri::{LogicalPosition, LogicalSize, Manager, Emitter};
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
        ])
        .setup(move |app| {
            let header_val = header_value.clone();

            let window = WindowBuilder::new(app, "main")
                .inner_size(WIN_W, WIN_H)
                .min_inner_size(800.0, 400.0)
                .decorations(false)
                .build()?;

            let chrome = window.add_child(
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
                ),
                LogicalPosition::new(0.0, CHROME_H),
                LogicalSize::new(WIN_W, WIN_H - CHROME_H),
            )?;

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
                })?;
            }

            // Header injection
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

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Onyxia failed to start");
}
