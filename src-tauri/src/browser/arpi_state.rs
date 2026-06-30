// Copyright (c) 2026 Edison Lepiten / AIEONYX
// SPDX-License-Identifier: Apache-2.0
//
// C13: ARPi connection state — now driven by AXON FFI verifier.
// Layer states are computed by axon_awp_ffi::verify_url(), not hardcoded.
// AXON-STUB-001: stub verifier active; enable axon-live for P45 mesh verifier.
// INVARIANT: frontend never computes protocol or ARPi state.
// Public terminology: AXON Receptor Protocol Interface.

use axon_awp_ffi::verify_url;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum LayerStatus {
    Verified,
    Pending,
    Failed,
    Inactive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArpiStatus {
    pub schema: LayerStatus,
    pub identity: LayerStatus,
    pub mutual_auth: LayerStatus,
    pub scope: LayerStatus,
    pub anomaly: LayerStatus,
}

impl ArpiStatus {
    /// Build ArpiStatus from AXON FFI result for a given URL.
    /// C13: this is the real FFI call path — stub verifier behind the scenes.
    pub fn from_axon_verify(url: &str) -> Self {
        let r = verify_url(url);
        ArpiStatus {
            schema:      if r.l1_schema    { LayerStatus::Verified } else { LayerStatus::Failed },
            identity:    if r.l2_identity  { LayerStatus::Verified } else { LayerStatus::Inactive },
            mutual_auth: if r.l3_auth      { LayerStatus::Verified } else { LayerStatus::Inactive },
            scope:       if r.l4_scope     { LayerStatus::Verified } else { LayerStatus::Inactive },
            anomaly:     if r.l5_anomaly   { LayerStatus::Verified } else { LayerStatus::Failed },
        }
    }

    /// Reserved for future trust-bar UI — not yet wired to a caller.
    #[allow(dead_code)]
    pub fn sovereign() -> Self {
        ArpiStatus {
            schema:      LayerStatus::Verified,
            identity:    LayerStatus::Verified,
            mutual_auth: LayerStatus::Verified,
            scope:       LayerStatus::Verified,
            anomaly:     LayerStatus::Verified,
        }
    }

    pub fn legacy() -> Self {
        ArpiStatus {
            schema:      LayerStatus::Inactive,
            identity:    LayerStatus::Inactive,
            mutual_auth: LayerStatus::Inactive,
            scope:       LayerStatus::Inactive,
            anomaly:     LayerStatus::Inactive,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Protocol {
    Https,
    Awp,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageState {
    pub protocol: Protocol,
    pub host: String,
    pub arpi: ArpiStatus,
}

impl PageState {
    pub fn blank() -> Self {
        PageState {
            protocol: Protocol::None,
            host: String::new(),
            arpi: ArpiStatus::legacy(),
        }
    }

    /// Build PageState from URL — ARPi layers driven by AXON FFI verifier.
    /// C13: real FFI call for every navigation. AXON-STUB-001 active.
    pub fn from_url(url: &str) -> Self {
        if url.starts_with("awp://") {
            let host = url.trim_start_matches("awp://")
                .split('/')
                .next()
                .unwrap_or("")
                .to_string();
            PageState {
                protocol: Protocol::Awp,
                host,
                arpi: ArpiStatus::from_axon_verify(url),
            }
        } else if url.starts_with("https://") || url.starts_with("http://") {
            let host = url
                .trim_start_matches("https://")
                .trim_start_matches("http://")
                .split('/')
                .next()
                .unwrap_or("")
                .to_string();
            PageState {
                protocol: Protocol::Https,
                host,
                arpi: ArpiStatus::from_axon_verify(url),
            }
        } else {
            PageState::blank()
        }
    }
}

