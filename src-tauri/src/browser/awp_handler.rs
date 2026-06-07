// Copyright (c) 2026 Edison Lepiten / AIEONYX
// SPDX-License-Identifier: Apache-2.0
//
// C4: awp:// protocol handler.
// Intercepts all awp:// requests and returns sovereign responses.
// Real AWP mesh routing: future milestone.
// INVARIANT: awp:// never falls through to legacy web.

use tauri::http::{Request, Response, StatusCode};

pub fn handle_awp_request(request: Request<Vec<u8>>) -> Response<Vec<u8>> {
    let url = request.uri().to_string();
    let host = extract_awp_host(&url);

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>AWP — {host}</title>
  <style>
    * {{ box-sizing: border-box; margin: 0; padding: 0; }}
    body {{
      background: #0a0a0f;
      color: #e8e8f0;
      font-family: "SF Pro Display", "Segoe UI", system-ui, sans-serif;
      display: flex;
      align-items: center;
      justify-content: center;
      min-height: 100vh;
      flex-direction: column;
      gap: 24px;
    }}
    .star {{ font-size: 48px; color: #00E5FF; }}
    h1 {{ font-size: 24px; font-weight: 600; color: #e8e8f0; }}
    .host {{ font-size: 14px; color: #7a7a9a; font-family: monospace; }}
    .status {{
      font-size: 13px;
      color: #7B2FBE;
      border: 1px solid #2a2a3a;
      padding: 8px 16px;
      border-radius: 6px;
    }}
  </style>
</head>
<body>
  <div class="star">✶</div>
  <h1>Sovereign Node</h1>
  <div class="host">awp://{host}</div>
  <div class="status">AWP mesh routing — coming in a future release</div>
</body>
</html>"#,
        host = host
    );

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .header("AXON-Client", "onyxia/0.1.0 (sovereign; linux)")
        .header("X-AWP-Handler", "onyxia-c4")
        .body(html.into_bytes())
        .unwrap()
}

fn extract_awp_host(url: &str) -> String {
    url.trim_start_matches("awp://")
        .split('/')
        .next()
        .unwrap_or("unknown")
        .to_string()
}
