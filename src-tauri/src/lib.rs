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
use tauri::Manager;

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
            let webview = app.get_webview_window("main")
                .expect("main webview window not found");

            #[cfg(target_os = "linux")]
            {
                use webkit2gtk::{URIRequestExt, WebResourceExt, WebViewExt};
                webview.with_webview(move |wv| {
                    let inner = wv.inner();
                    let hv = header_val.clone();
                    inner.connect_resource_load_started(
                        move |_view,
                              resource: &webkit2gtk::WebResource,
                              request: &webkit2gtk::URIRequest| {
                            if let Some(url) = resource.uri() {
                                if should_inject_header(url.as_str()) {
                                    if let Some(headers) = request.http_headers() {
                                        headers.append(AXON_CLIENT_HEADER, hv.as_str());
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
