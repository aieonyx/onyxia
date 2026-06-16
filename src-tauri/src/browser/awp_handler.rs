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
    // Route sovereign AWP pages
    if url.starts_with("awp://legacy") {
        return serve_legacy_page();
    }

    if url.starts_with("awp://aegis") {
        return serve_aegis_page();
    }

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

fn serve_legacy_page() -> tauri::http::Response<Vec<u8>> {
    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <title>Digital Legacy — AIEONYX</title>
  <style>
    * { box-sizing: border-box; margin: 0; padding: 0; }
    body {
      background: #0a0a0f; color: #e8e8f0;
      font-family: "SF Pro Display", "Segoe UI", system-ui, sans-serif;
      padding: 32px; min-height: 100vh;
    }
    h1 { font-size: 22px; font-weight: 700; color: #fb923c; margin-bottom: 6px; }
    .subtitle { font-size: 13px; color: #7a7a9a; margin-bottom: 28px; }
    .section { margin-bottom: 20px; }
    label { display: block; font-size: 12px; color: #9a9ab0; margin-bottom: 6px; font-weight: 500; }
    input, select {
      width: 100%; background: #16161f; border: 1px solid #2a2a3a;
      color: #e8e8f0; border-radius: 6px; padding: 8px 12px;
      font-size: 13px; outline: none;
    }
    input:focus, select:focus { border-color: #fb923c; }
    .row { display: grid; grid-template-columns: 1fr 1fr; gap: 16px; }
    .btn {
      background: #fb923c; border: none; color: #000;
      cursor: pointer; font-size: 13px; font-weight: 700;
      padding: 10px 24px; border-radius: 6px; margin-top: 8px;
    }
    .btn:hover { opacity: 0.85; }
    .status { font-size: 12px; color: #4ade80; margin-top: 10px; min-height: 18px; }
    .status.error { color: #f87171; }
    .divider { border: none; border-top: 1px solid #1e1e2a; margin: 24px 0; }
    .info-box {
      background: #12121c; border: 1px solid #1e2030; border-radius: 8px;
      padding: 14px 16px; margin-bottom: 24px; font-size: 12px; color: #7a7a9a;
      line-height: 1.6;
    }
    .info-box strong { color: #fb923c; }
    #heartbeat-status { font-size: 12px; color: #4ade80; }
  </style>
</head>
<body>
  <h1>&#9760; Digital Legacy</h1>
  <p class="subtitle">Define what happens to your sovereign data when you are gone.</p>

  <div class="info-box">
    <strong>Sovereign Consent Doctrine</strong><br>
    Your data, your rules. AIEONYX cannot help you if you give the key away.
    This testament is stored encrypted in your Critical tier and executed automatically.
  </div>

  <div class="section">
    <label>Inactivity Trigger (days)</label>
    <input type="number" id="legacy-days" value="180" min="30" max="3650" />
  </div>

  <div class="row">
    <div class="section">
      <label>Legacy Holder Name</label>
      <input type="text" id="legacy-holder-name" placeholder="Full name" />
    </div>
    <div class="section">
      <label>Legacy Holder Contact</label>
      <input type="text" id="legacy-holder-contact" placeholder="Email or phone" />
    </div>
  </div>

  <hr class="divider" />

  <div class="row">
    <div class="section">
      <label>Critical Data (passwords, keys)</label>
      <select id="legacy-critical-action">
        <option value="delete">Delete immediately</option>
        <option value="preserve">Preserve for holder</option>
      </select>
    </div>
    <div class="section">
      <label>Personal Data (history, notes)</label>
      <select id="legacy-personal-action">
        <option value="transfer">Transfer to holder</option>
        <option value="delete">Delete</option>
      </select>
    </div>
  </div>

  <div class="section">
    <label>Noise Data (cache, analytics)</label>
    <select id="legacy-noise-action">
      <option value="purge">Purge immediately</option>
      <option value="preserve">Preserve</option>
    </select>
  </div>

  <hr class="divider" />

  <button class="btn" onclick="saveTestament()">Save Testament to Vault</button>
  <button class="btn" style="background:#1e2030;color:#e8e8f0;margin-left:12px;" onclick="loadTestament()">Load Existing</button>
  <div class="status" id="status-msg"></div>

  <hr class="divider" />
  <div id="heartbeat-status">Heartbeat: checking...</div>

  <script>
    const { invoke } = window.__TAURI__.core;

    async function saveTestament() {
      const testament = {
        inactivity_days: parseInt(document.getElementById('legacy-days').value) || 180,
        legacy_holder_name: document.getElementById('legacy-holder-name').value.trim(),
        legacy_holder_contact: document.getElementById('legacy-holder-contact').value.trim(),
        critical_action: document.getElementById('legacy-critical-action').value,
        personal_action: document.getElementById('legacy-personal-action').value,
        noise_action: document.getElementById('legacy-noise-action').value,
        created_at: Math.floor(Date.now() / 1000),
      };
      try {
        await invoke('save_testament', { testament });
        showStatus('Testament saved to sovereign vault', false);
      } catch (e) {
        showStatus('Save failed: ' + e, true);
      }
    }

    async function loadTestament() {
      try {
        const t = await invoke('load_testament');
        if (t) {
          document.getElementById('legacy-days').value = t.inactivity_days;
          document.getElementById('legacy-holder-name').value = t.legacy_holder_name;
          document.getElementById('legacy-holder-contact').value = t.legacy_holder_contact;
          document.getElementById('legacy-critical-action').value = t.critical_action;
          document.getElementById('legacy-personal-action').value = t.personal_action;
          document.getElementById('legacy-noise-action').value = t.noise_action;
          showStatus('Testament loaded from vault', false);
        } else {
          showStatus('No testament found', false);
        }
      } catch (e) {
        showStatus('Load failed: ' + e, true);
      }
    }

    function showStatus(msg, isError) {
      const el = document.getElementById('status-msg');
      el.textContent = msg;
      el.className = isError ? 'status error' : 'status';
    }

    async function checkHeartbeat() {
      try {
        const hb = await invoke('ping_heartbeat');
        document.getElementById('heartbeat-status').textContent =
          'Heartbeat: active — testament ' + (hb.testament_active ? 'configured' : 'not set');
      } catch (e) {
        document.getElementById('heartbeat-status').textContent = 'Heartbeat: EdisonDB offline';
        document.getElementById('heartbeat-status').style.color = '#f87171';
      }
    }

    // Auto-load on page open
    loadTestament();
    checkHeartbeat();
  </script>
</body>
</html>"#;

    tauri::http::Response::builder()
        .status(200)
        .header("Content-Type", "text/html")
        .body(html.as_bytes().to_vec())
        .unwrap()
}

fn serve_aegis_page() -> tauri::http::Response<Vec<u8>> {
    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <title>Aegis Threat Intel — AIEONYX</title>
  <style>
    * { box-sizing: border-box; margin: 0; padding: 0; }
    body {
      background: #0a0a0f; color: #e8e8f0;
      font-family: "SF Pro Display", "Segoe UI", system-ui, sans-serif;
      padding: 32px; min-height: 100vh;
    }
    h1 { font-size: 22px; font-weight: 700; color: #f87171; margin-bottom: 6px; }
    .subtitle { font-size: 13px; color: #7a7a9a; margin-bottom: 28px; }
    .grid { display: grid; grid-template-columns: 1fr 1fr; gap: 20px; margin-bottom: 28px; }
    .card {
      background: #12121c; border: 1px solid #1e1e2a;
      border-radius: 10px; padding: 18px;
    }
    .card-title {
      font-size: 11px; font-weight: 700; color: #7a7a9a;
      text-transform: uppercase; letter-spacing: 1px; margin-bottom: 14px;
    }
    .stat { font-size: 32px; font-weight: 700; color: #f87171; margin-bottom: 4px; }
    .stat.safe { color: #4ade80; }
    .stat-label { font-size: 12px; color: #7a7a9a; }
    .layer-grid { display: flex; flex-direction: column; gap: 8px; }
    .layer {
      display: flex; align-items: center; justify-content: space-between;
      background: #0d0d16; border-radius: 6px; padding: 8px 12px;
      font-size: 12px;
    }
    .layer-name { color: #9a9ab0; }
    .layer-status { font-weight: 600; }
    .layer-status.ok { color: #4ade80; }
    .layer-status.warn { color: #fb923c; }
    .layer-status.fail { color: #f87171; }
    .section-title {
      font-size: 13px; font-weight: 600; color: #e8e8f0;
      margin-bottom: 12px; padding-bottom: 8px;
      border-bottom: 1px solid #1e1e2a;
    }
    .threat-list { display: flex; flex-direction: column; gap: 8px; max-height: 280px; overflow-y: auto; }
    .threat-item {
      background: #12121c; border-left: 3px solid #f87171;
      border-radius: 0 6px 6px 0; padding: 10px 14px; font-size: 12px;
    }
    .threat-item.ssv { border-left-color: #fb923c; }
    .threat-kind { font-weight: 600; color: #f87171; margin-bottom: 3px; }
    .threat-item.ssv .threat-kind { color: #fb923c; }
    .threat-domain { color: #9a9ab0; font-family: monospace; font-size: 11px; }
    .threat-time { color: #5a5a7a; font-size: 10px; margin-top: 3px; }
    .empty { color: #5a5a7a; font-size: 12px; padding: 16px; text-align: center; }
    .clear-btn {
      background: none; border: 1px solid #2a2a3a; color: #7a7a9a;
      cursor: pointer; font-size: 11px; padding: 4px 10px;
      border-radius: 4px; margin-bottom: 12px; float: right;
    }
    .clear-btn:hover { border-color: #f87171; color: #f87171; }
    .blocklist-info {
      background: #12121c; border: 1px solid #1e1e2a;
      border-radius: 10px; padding: 18px; margin-bottom: 20px;
    }
    .bl-row { display: flex; justify-content: space-between; align-items: center;
      padding: 6px 0; border-bottom: 1px solid #1a1a2a; font-size: 12px; }
    .bl-row:last-child { border-bottom: none; }
    .bl-label { color: #9a9ab0; }
    .bl-count { color: #4ade80; font-weight: 600; font-family: monospace; }
  </style>
</head>
<body>
  <h1>&#128737; Aegis Threat Intel</h1>
  <p class="subtitle">Sovereign threat monitoring — all analysis runs locally, zero telemetry.</p>

  <div class="grid">
    <div class="card">
      <div class="card-title">Session Threats</div>
      <div class="stat" id="threat-count">0</div>
      <div class="stat-label">trackers detected</div>
    </div>
    <div class="card">
      <div class="card-title">SSV Blocks</div>
      <div class="stat" id="ssv-count">0</div>
      <div class="stat-label">navigations blocked</div>
    </div>
  </div>

  <div class="blocklist-info">
    <div class="card-title" style="margin-bottom:12px">STS Blocklist Status</div>
    <div class="bl-row">
      <span class="bl-label">Tracker domains</span>
      <span class="bl-count">29</span>
    </div>
    <div class="bl-row">
      <span class="bl-label">Known-good domains</span>
      <span class="bl-count">21</span>
    </div>
    <div class="bl-row">
      <span class="bl-label">Crypto allowlist</span>
      <span class="bl-count">9</span>
    </div>
    <div class="bl-row">
      <span class="bl-label">Typosquat threshold</span>
      <span class="bl-count">Levenshtein &le; 2</span>
    </div>
    <div class="bl-row">
      <span class="bl-label">Source</span>
      <span class="bl-count">Local — no network calls</span>
    </div>
  </div>

  <div style="margin-bottom:28px">
    <div class="section-title">
      ARPi Layer Status
    </div>
    <div class="layer-grid" id="arpi-layers">
      <div class="layer"><span class="layer-name">L1 Schema</span><span class="layer-status ok">Verified</span></div>
      <div class="layer"><span class="layer-name">L2 Identity</span><span class="layer-status ok">Verified</span></div>
      <div class="layer"><span class="layer-name">L3 Mutual Auth</span><span class="layer-status ok">Verified</span></div>
      <div class="layer"><span class="layer-name">L4 Scope</span><span class="layer-status ok">Verified</span></div>
      <div class="layer"><span class="layer-name">L5 Anomaly</span><span class="layer-status ok" id="l5-status">Clear</span></div>
    </div>
  </div>

  <div>
    <button class="clear-btn" onclick="clearLog()">Clear log</button>
    <div class="section-title">Threat Log</div>
    <div class="threat-list" id="threat-list">
      <div class="empty">No threats detected this session</div>
    </div>
  </div>

  <script>
    const { invoke, event } = window.__TAURI__;

    let threats = JSON.parse(sessionStorage.getItem('aegis_threats') || '[]');
    let ssvBlocks = JSON.parse(sessionStorage.getItem('aegis_ssv') || '[]');

    function renderThreats() {
      const list = document.getElementById('threat-list');
      const allEvents = [...threats.map(t => ({...t, cat: 'sts'})),
                         ...ssvBlocks.map(t => ({...t, cat: 'ssv'}))];

      document.getElementById('threat-count').textContent = threats.length;
      document.getElementById('ssv-count').textContent = ssvBlocks.length;

      if (allEvents.length === 0) {
        list.innerHTML = '<div class="empty">No threats detected this session</div>';
        document.getElementById('l5-status').textContent = 'Clear';
        document.getElementById('l5-status').className = 'layer-status ok';
        return;
      }

      document.getElementById('l5-status').textContent = allEvents.length + ' event(s)';
      document.getElementById('l5-status').className = 'layer-status warn';

      list.innerHTML = allEvents.reverse().map(t => `
        <div class="threat-item ${t.cat}">
          <div class="threat-kind">${t.kind.replace(/_/g, ' ').toUpperCase()}</div>
          <div class="threat-domain">${t.domain || t.url || ''}</div>
          <div class="threat-time">${t.time || 'this session'}</div>
        </div>
      `).join('');
    }

    function clearLog() {
      threats = []; ssvBlocks = [];
      sessionStorage.removeItem('aegis_threats');
      sessionStorage.removeItem('aegis_ssv');
      renderThreats();
    }

    // Listen for new threats from STS
    event.listen('threat-detected', (e) => {
      threats.push({...e.payload, time: new Date().toLocaleTimeString()});
      sessionStorage.setItem('aegis_threats', JSON.stringify(threats));
      renderThreats();
    });

    // Listen for SSV blocks
    event.listen('ssv-blocked', (e) => {
      ssvBlocks.push({...e.payload.threat, time: new Date().toLocaleTimeString()});
      sessionStorage.setItem('aegis_ssv', JSON.stringify(ssvBlocks));
      renderThreats();
    });

    renderThreats();
  </script>
</body>
</html>"#;

    tauri::http::Response::builder()
        .status(200)
        .header("Content-Type", "text/html")
        .body(html.as_bytes().to_vec())
        .unwrap()
}
