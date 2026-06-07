// Copyright (c) 2026 Edison Lepiten / AIEONYX
// SPDX-License-Identifier: Apache-2.0
//
// C3: AXON-Client header injection.
// INVARIANT: header grants no elevated server permissions (SI-5).
// INVARIANT: injection never originates from frontend TypeScript.

pub const AXON_CLIENT_HEADER: &str = "AXON-Client";

pub fn axon_client_value() -> String {
    let version = env!("CARGO_PKG_VERSION");
    let os = axon_os_string();
    format!("onyxia/{} (sovereign; {})", version, os)
}

fn axon_os_string() -> &'static str {
    #[cfg(target_os = "linux")]
    return "linux";
    #[cfg(target_os = "macos")]
    return "macos";
    #[cfg(target_os = "windows")]
    return "windows";
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    return "unknown";
}

pub fn should_inject_header(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}
