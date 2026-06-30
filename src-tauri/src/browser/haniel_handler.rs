// Copyright (c) 2026 Edison Lepiten / AIEONYX
// SPDX-License-Identifier: Apache-2.0
//
// HE-15b: HANIEL render-bridge protocol handler.
// `haniel://` is NOT a peer protocol to https:// or awp:// — it is purely
// internal plumbing, never typed by a user or shown in the address bar.
// Its only job: take a URL HERALD already fetched and HANIEL already
// rendered (whether that URL was https:// legacy web or awp:// sovereign
// content), and hand the resulting pixels to the `content` Tauri webview
// as something it can actually display, since Tauri has no API to paint
// a native Rust pixel buffer into a webview surface directly.
//
// Two routes under this scheme:
//   haniel://render?url=<percent-encoded original URL>
//     -> minimal HTML wrapper page, full-bleed <img> pointing at /frame
//   haniel://frame?url=<percent-encoded original URL>
//     -> the actual PNG bytes for that URL, via PageLoader + encode_png
//
// INVARIANT: this handler never fetches a URL itself — all fetching and
// threat-gating happens inside PageLoader (HERALD), which this only calls.

use haniel::PageLoader;
use haniel_canvas::encode_png;
use std::sync::Mutex;
use tauri::http::{Request, Response, StatusCode};

/// Shared HANIEL page loader — one instance per app, matching the existing
/// Arc<Mutex<TabManager>> managed-state pattern already used in lib.rs.
pub struct HanielState(pub Mutex<PageLoader>);

impl HanielState {
    pub fn new(width: u32, height: u32) -> Self {
        Self(Mutex::new(PageLoader::new(width, height)))
    }
}

/// Build a `haniel://render?url=...` URI for a given target URL.
/// This is what `commands::navigation::navigate` points the content
/// webview at, instead of navigating it to the target URL directly.
pub fn to_render_uri(target_url: &str) -> String {
    format!("haniel://render?url={}", percent_encode(target_url))
}

/// Handle a `haniel://` request — dispatches to the render-page or
/// frame-bytes route based on path.
pub fn handle_haniel_request(
    state: &HanielState,
    request: Request<Vec<u8>>,
) -> Response<Vec<u8>> {
    let uri = request.uri().to_string();

    let target_url = match extract_target_url(&uri) {
        Some(u) => u,
        None => return bad_request("missing or malformed url= parameter"),
    };

    if uri.contains("/frame") {
        serve_frame(state, &target_url)
    } else {
        serve_render_page(&target_url)
    }
}

/// Extract and percent-decode the `url=` query parameter from a
/// `haniel://render?url=...` or `haniel://frame?url=...` request URI.
///
/// Also used outside this module (lib.rs's WebKitGTK uri-notify hook) to
/// unwrap the haniel:// render-bridge wrapper back to the real page URL
/// for address-bar display — the webview's own internal URI is the
/// wrapper, not the page the user actually navigated to.
pub fn extract_target_url(uri: &str) -> Option<String> {
    let query_start = uri.find('?')? + 1;
    let query = &uri[query_start..];

    for pair in query.split('&') {
        if let Some(encoded) = pair.strip_prefix("url=") {
            return Some(percent_decode(encoded));
        }
    }
    None
}

/// Minimal percent-decoding — handles %XX escapes. Sufficient for URL
/// query parameters; not a general-purpose decoder.
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

/// Serve the HTML wrapper page that displays the rendered frame full-bleed.
fn serve_render_page(target_url: &str) -> Response<Vec<u8>> {
    let frame_src = format!("frame?url={}", percent_encode(target_url));

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <title>HANIEL — {url}</title>
  <style>
    * {{ box-sizing: border-box; margin: 0; padding: 0; }}
    html, body {{ width: 100%; height: 100%; background: #121218; overflow: hidden; }}
    img {{ display: block; width: 100%; height: 100%; object-fit: contain; }}
  </style>
</head>
<body>
  <img src="{frame_src}" alt="HANIEL sovereign render" />
</body>
</html>"#,
        url = escape_html(target_url),
        frame_src = frame_src,
    );

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .header("Cache-Control", "no-store")
        .header(
            "Content-Security-Policy",
            "default-src 'none'; img-src 'self'; style-src 'unsafe-inline'",
        )
        .body(html.into_bytes())
        .unwrap()
}

/// Serve the actual rendered PNG bytes for a URL via the HANIEL pipeline.
fn serve_frame(state: &HanielState, target_url: &str) -> Response<Vec<u8>> {
    let loader = match state.0.lock() {
        Ok(l) => l,
        Err(_) => return server_error("page loader lock poisoned"),
    };

    let result = match loader.load_full(target_url) {
        Ok(r) => r,
        Err(e) => return server_error(&format!("HANIEL render failed: {}", e)),
    };

    let png = match encode_png(&result.pixels) {
        Ok(bytes) => bytes,
        Err(e) => return server_error(&format!("PNG encode failed: {}", e)),
    };

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/png")
        .header("Cache-Control", "no-store")
        .header("X-Content-Type-Options", "nosniff")
        .header("X-Haniel-Arpi-Tier", format!("{:?}", result.arpi_tier))
        .header("X-Haniel-Threat-Verdict", format!("{:?}", result.threat_verdict))
        .body(png)
        .unwrap()
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

/// Minimal percent-encoding for embedding a URL inside a query parameter.
/// Encodes the characters that would otherwise break query-string parsing
/// or HTML attribute syntax.
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
        let uri = to_render_uri("https://example.com/");
        assert!(uri.starts_with("haniel://render?url="));
    }

    #[test]
    fn to_render_uri_is_decodable_back_to_original() {
        let original = "awp://aegis";
        let uri = to_render_uri(original);
        let decoded = extract_target_url(&uri).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn extract_target_url_basic() {
        let uri = "haniel://render?url=https%3A%2F%2Fexample.com%2F";
        assert_eq!(
            extract_target_url(uri),
            Some("https://example.com/".to_string())
        );
    }

    #[test]
    fn extract_target_url_missing_param_is_none() {
        let uri = "haniel://render?foo=bar";
        assert_eq!(extract_target_url(uri), None);
    }

    #[test]
    fn extract_target_url_no_query_is_none() {
        let uri = "haniel://render";
        assert_eq!(extract_target_url(uri), None);
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
        // Malformed trailing %, should not panic, just pass through
        assert_eq!(percent_decode("abc%"), "abc%");
    }

    #[test]
    fn percent_encode_roundtrips_with_decode() {
        let original = "https://example.com/path?a=b&c=d";
        let encoded = percent_encode(original);
        let decoded = percent_decode(&encoded);
        assert_eq!(decoded, original);
    }

    #[test]
    fn percent_encode_leaves_safe_chars_alone() {
        assert_eq!(percent_encode("abc123-_.~"), "abc123-_.~");
    }

    #[test]
    fn escape_html_escapes_special_chars() {
        let escaped = escape_html(r#"<script>"x"&y</script>"#);
        assert!(!escaped.contains('<'));
        assert!(!escaped.contains('>'));
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
        let resp = serve_render_page("https://example.com/<script>");
        let body = String::from_utf8(resp.body().clone()).unwrap();
        assert!(!body.contains("<script>"));
    }

    #[test]
    fn serve_frame_sovereign_awp_page_returns_png() {
        let state = HanielState::new(800, 600);
        let resp = serve_frame(&state, "awp://aegis");
        assert_eq!(resp.status(), StatusCode::OK);
        let body = resp.body();
        assert_eq!(&body[0..8], &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]);
    }

    #[test]
    fn serve_frame_sets_png_content_type() {
        let state = HanielState::new(800, 600);
        let resp = serve_frame(&state, "awp://aegis");
        assert_eq!(
            resp.headers().get("Content-Type").unwrap(),
            "image/png"
        );
    }

    #[test]
    fn serve_frame_blocked_tracker_returns_server_error() {
        let state = HanielState::new(800, 600);
        let resp = serve_frame(&state, "https://doubleclick.net/");
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn handle_haniel_request_render_route() {
        let state = HanielState::new(800, 600);
        let req = Request::builder()
            .uri("haniel://render?url=awp%3A%2F%2Faegis")
            .body(Vec::new())
            .unwrap();
        let resp = handle_haniel_request(&state, req);
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get("Content-Type").unwrap(),
            "text/html; charset=utf-8"
        );
    }

    #[test]
    fn handle_haniel_request_frame_route() {
        let state = HanielState::new(800, 600);
        let req = Request::builder()
            .uri("haniel://frame?url=awp%3A%2F%2Faegis")
            .body(Vec::new())
            .unwrap();
        let resp = handle_haniel_request(&state, req);
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get("Content-Type").unwrap(),
            "image/png"
        );
    }

    #[test]
    fn handle_haniel_request_missing_url_is_bad_request() {
        let state = HanielState::new(800, 600);
        let req = Request::builder()
            .uri("haniel://render")
            .body(Vec::new())
            .unwrap();
        let resp = handle_haniel_request(&state, req);
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}
