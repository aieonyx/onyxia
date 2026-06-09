// Copyright (c) 2026 Edison Lepiten / AIEONYX
// SPDX-License-Identifier: Apache-2.0
//
// C5-A: ARPi connection state.
// Computed in Rust main process. Frontend renders only.
// Public terminology: AXON Receptor Protocol Interface.

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
    pub fn sovereign() -> Self {
        ArpiStatus {
            schema: LayerStatus::Verified,
            identity: LayerStatus::Verified,
            mutual_auth: LayerStatus::Verified,
            scope: LayerStatus::Verified,
            anomaly: LayerStatus::Verified,
        }
    }
    pub fn legacy() -> Self {
        ArpiStatus {
            schema: LayerStatus::Inactive,
            identity: LayerStatus::Inactive,
            mutual_auth: LayerStatus::Inactive,
            scope: LayerStatus::Inactive,
            anomaly: LayerStatus::Inactive,
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

    pub fn from_url(url: &str) -> Self {
        if url.starts_with("awp://") {
            let host = url.trim_start_matches("awp://")
                .split('/')
                .next()
                .unwrap_or("")
                .to_string();
            PageState { protocol: Protocol::Awp, host, arpi: ArpiStatus::sovereign() }
        } else if url.starts_with("https://") || url.starts_with("http://") {
            let host = url
                .trim_start_matches("https://")
                .trim_start_matches("http://")
                .split('/')
                .next()
                .unwrap_or("")
                .to_string();
            PageState { protocol: Protocol::Https, host, arpi: ArpiStatus::legacy() }
        } else {
            PageState::blank()
        }
    }
}
