// Copyright (c) 2026 Edison Lepiten / AIEONYX
// SPDX-License-Identifier: Apache-2.0
//
// TrustIndicator — computed by main process, pushed to frontend.
// Frontend NEVER determines trust state. Frontend renders what it receives.
// C1: all sites are Legacy until C3/C6 are implemented.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TrustIndicator {
    /// C1 placeholder — no indicator computed yet
    Unknown,
    /// Standard HTTPS — padlock
    LegacyHttps,
    /// Unencrypted HTTP — warning
    LegacyHttp,
    /// AXON Web active — sovereign star (C6)
    SovereignStar,
    /// AIEONYX CA cert — blue padlock (C6)
    AieonxCa,
}

impl Default for TrustIndicator {
    fn default() -> Self {
        TrustIndicator::Unknown
    }
}
