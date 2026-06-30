// Copyright (c) 2026 Edison Lepiten / AIEONYX
// SPDX-License-Identifier: Apache-2.0
//
// C14: AIEONYX CA pre-installation.
// The AIEONYX Root CA certificate is embedded in the binary at compile time.
//
// HE-15c note: this module's doc comments previously described registering
// the CA with "WebKitGTK's TLS database" — that never actually happened.
// webkit2gtk's Rust bindings never exposed `set_tls_database()` (AUDIT-002),
// so the real behavior was always the fallback below: write the PEM to the
// user config dir for manual import. The unused `&tauri::Webview` parameter
// (a leftover from the originally-planned WebKitGTK call) has been dropped.
// Actual TLS/CA trust resolution for sovereign content lives in HANIEL
// HERALD's ArpiResolver (see haniel_handler::serve_frame, which surfaces
// the resolved ArpiTier as a response header), not here.
//
// CA details:
//   Subject: CN=AIEONYX Root CA, O=AIEONYX, OU=Sovereign Infrastructure
//   Algorithm: Ed25519
//   Validity: 2026-06-18 — 2036-06-15 (10 years)
//   SHA-256: 64:6D:99:AD:46:C2:50:A8:83:A1:95:C1:36:25:0B:1E:
//            47:EC:5F:BE:CB:90:A2:C4:C0:28:DC:8F:8C:B7:8A:89
//   SHA-1:   77:44:37:A5:68:5B:CF:9C:93:C7:C9:9D:91:CA:BE:DB:01:ED:87:30

/// AIEONYX Root CA — PEM, embedded at compile time.
pub const AIEONYX_CA_PEM: &str = include_str!("aieonyx_ca.pem");

/// SHA-256 fingerprint of the AIEONYX Root CA.
/// Used to identify AIEONYX-signed connections and set TrustIndicator::AieonxCa.
pub const AIEONYX_CA_FINGERPRINT_SHA256: &str =
    "64:6D:99:AD:46:C2:50:A8:83:A1:95:C1:36:25:0B:1E:47:EC:5F:BE:CB:90:A2:C4:C0:28:DC:8F:8C:B7:8A:89";

/// Write the AIEONYX Root CA to the user config dir for manual import
/// into the system trust store. Called once during app setup.
pub fn install_aieonyx_ca() -> Result<(), String> {
    let config_dir = dirs_next::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
        .join("onyxia");
    let _ = std::fs::create_dir_all(&config_dir);
    let ca_path = config_dir.join("aieonyx_ca.pem");
    if let Err(e) = std::fs::write(&ca_path, AIEONYX_CA_PEM) {
        log::warn!("C14: could not write CA PEM to config dir: {e}");
    } else {
        log::info!("C14: AIEONYX Root CA written to {}", ca_path.display());
    }

    log::info!(
        "C14: AIEONYX Root CA embedded. SHA-256: {}",
        AIEONYX_CA_FINGERPRINT_SHA256
    );
    Ok(())
}
