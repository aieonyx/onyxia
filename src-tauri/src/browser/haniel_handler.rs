// Copyright (c) 2026 Edison Lepiten / AIEONYX
// SPDX-License-Identifier: Apache-2.0

use haniel::PageLoader;
use haniel_canvas::encode_png;
use std::sync::Mutex;
use tauri::http::{Request, Response, StatusCode};
use tauri::{AppHandle, Emitter};

pub struct HanielState {
    loader: Mutex<PageLoader>,
    axon_client_value: String,
}

impl HanielState {
    pub fn new(width: u32, height: u32, axon_client_value: String) -> Self {
        Self {
            loader: Mutex::new(PageLoader::new(width, height)),
            axon_client_value,
        }
    }
}

/// Build the internal haniel:// render URI for a target URL.
pub fn to_render_uri(target_url: &str) -> String {
    format!("haniel://render?url={}", percent_encode(target_url))
}

pub fn handle_haniel_request(
    state: &HanielState,
    app: &AppHandle,
    request: Request<Vec<u8>>,
) -> Response<Vec<u8>> {
    let uri = request.uri().to_string();
    let target_url = match extract_target_url(&uri) {
        Some(u) => u,
        None => return bad_request("missing url= parameter"),
    };
    if uri.contains("/frame") {
        serve_frame(state, app, &target_url)
    } else {
        serve_render_page(&target_url)
    }
}

pub fn extract_target_url(uri: &str) -> Option<String> {
    let query = &uri[uri.find('?')? + 1..];
    for pair in query.split('&') {
        if let Some(encoded) = pair.strip_prefix("url=") {
            return Some(percent_decode(encoded));
        }
    }
    None
}

fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(hex) = std::str::from_utf8(&bytes[i + 1..i + 3]) {
                if let Ok(byte) = u8::from_str_radix(hex, 16) {
                    out.push(byte);
                    i += 3;
                    continue;
                }
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn serve_render_page(target_url: &str) -> Response<Vec<u8>> {
    let frame_src = format!("frame?url={}", percent_encode(target_url));
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <title>{url}</title>
  <style>
    * {{ box-sizing: border-box; margin: 0; padding: 0; }}
    html, body {{ width: 100%; height: 100%; background: #121218; overflow: hidden; }}
    img {{ display: block; width: 100%; height: 100%; object-fit: contain; }}
  </style>
</head>
<body>
  <img src="{frame_src}" alt="" />
</body>
</html>"#,
        url = escape_html(target_url),
        frame_src = frame_src,
    );
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .header("Cache-Control", "no-store")
        .header("Content-Security-Policy", "default-src 'none'; img-src 'self'; style-src 'unsafe-inline'")
        .body(html.into_bytes())
        .unwrap()
}

fn serve_frame(state: &HanielState, app: &AppHandle, target_url: &str) -> Response<Vec<u8>> {
    let loader = match state.loader.lock() {
        Ok(l) => l,
        Err(_) => return server_error("loader lock poisoned"),
    };
    let result = match loader.load_full(target_url) {
        Ok(r) => r,
        Err(e) => return server_error(&format!("render failed: {}", e)),
    };
    let png = match encode_png(&result.pixels) {
        Ok(b) => b,
        Err(e) => return server_error(&format!("encode failed: {}", e)),
    };

    if let Some(threat) = crate::browser::sts::is_tracker(target_url) {
        log::warn!("STS: {} {}", threat.kind, target_url);
        let _ = app.emit("threat-detected", &threat);
    }

    let _ = app.emit("url-changed", target_url);
    let title = crate::browser::sts::extract_host(target_url)
        .unwrap_or_else(|| target_url.to_string());
    let _ = app.emit("title-changed", title);

    let mut builder = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/png")
        .header("Cache-Control", "no-store")
        .header("X-Content-Type-Options", "nosniff")
        .header("X-Haniel-Arpi-Tier", format!("{:?}", result.arpi_tier))
        .header("X-Haniel-Threat-Verdict", format!("{:?}", result.threat_verdict));

    if crate::browser::header_injection::should_inject_header(target_url) {
        builder = builder.header(
            crate::browser::header_injection::AXON_CLIENT_HEADER,
            state.axon_client_value.as_str(),
        );
    }

    builder.body(png).unwrap()
}

fn bad_request(msg: &str) -> Response<Vec<u8>> {
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .header("Content-Type", "text/plain; charset=utf-8")
        .body(msg.as_bytes().to_vec())
        .unwrap()
}

fn server_error(msg: &str) -> Response<Vec<u8>> {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header("Content-Type", "text/plain; charset=utf-8")
        .body(msg.as_bytes().to_vec())
        .unwrap()
}

fn percent_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char);
            }
            _ => out.push_str(&format!("%{:02X}", b)),
        }
    }
    out
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_render_uri_wraps_target_url() {
        assert!(to_render_uri("https://example.com/").starts_with("haniel://render?url="));
    }

    #[test]
    fn to_render_uri_is_decodable_back_to_original() {
        let original = "awp://aegis";
        assert_eq!(extract_target_url(&to_render_uri(original)).unwrap(), original);
    }

    #[test]
    fn extract_target_url_basic() {
        let uri = "haniel://render?url=https%3A%2F%2Fexample.com%2F";
        assert_eq!(extract_target_url(uri), Some("https://example.com/".to_string()));
    }

    #[test]
    fn extract_target_url_missing_param_is_none() {
        assert_eq!(extract_target_url("haniel://render?foo=bar"), None);
    }

    #[test]
    fn extract_target_url_no_query_is_none() {
        assert_eq!(extract_target_url("haniel://render"), None);
    }

    #[test]
    fn percent_decode_handles_escapes() {
        assert_eq!(percent_decode("a%20b"), "a b");
        assert_eq!(percent_decode("https%3A%2F%2Fx.com"), "https://x.com");
    }

    #[test]
    fn percent_decode_passes_through_plain_text() {
        assert_eq!(percent_decode("hello"), "hello");
    }

    #[test]
    fn percent_decode_trailing_percent_no_panic() {
        assert_eq!(percent_decode("abc%"), "abc%");
    }

    #[test]
    fn percent_encode_roundtrips_with_decode() {
        let original = "https://example.com/path?a=b&c=d";
        assert_eq!(percent_decode(&percent_encode(original)), original);
    }

    #[test]
    fn percent_encode_leaves_safe_chars_alone() {
        assert_eq!(percent_encode("abc123-_.~"), "abc123-_.~");
    }

    #[test]
    fn escape_html_escapes_special_chars() {
        let escaped = escape_html(r#"<script>"x"&y</script>"#);
        assert!(!escaped.contains('<'));
        assert!(escaped.contains("&lt;"));
        assert!(escaped.contains("&quot;"));
        assert!(escaped.contains("&amp;"));
    }

    #[test]
    fn serve_render_page_returns_html() {
        let resp = serve_render_page("https://example.com/");
        assert_eq!(resp.status(), StatusCode::OK);
        let body = String::from_utf8(resp.body().clone()).unwrap();
        assert!(body.contains("<img"));
        assert!(body.contains("frame?url="));
    }

    #[test]
    fn serve_render_page_escapes_url_in_title() {
        let body = String::from_utf8(
            serve_render_page("https://example.com/<script>").body().clone()
        ).unwrap();
        assert!(!body.contains("<script>"));
    }
}
