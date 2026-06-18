// Copyright (c) 2026 Edison Lepiten / AIEONYX
// SPDX-License-Identifier: Apache-2.0
//
// C14: TrustIndicator — computed by Rust backend, pushed to frontend.
// Frontend NEVER determines trust state. Frontend renders what it receives.
// AieonxCa: set when connection is verified against AIEONYX Root CA (C14).

use crate::ca::AIEONYX_CA_FINGERPRINT_SHA256;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TrustIndicator {
    /// No indicator computed yet
    Unknown,
    /// Standard HTTPS — padlock
    LegacyHttps,
    /// Unencrypted HTTP — warning
    LegacyHttp,
    /// AWP sovereign page — sovereign star
    SovereignStar,
    /// AIEONYX CA signed — sovereign star + blue ring
    AieonxCa,
}

impl Default for TrustIndicator {
    fn default() -> Self {
        TrustIndicator::Unknown
    }
}

impl TrustIndicator {
    /// Derive trust indicator from URL and optional TLS cert fingerprint.
    pub fn from_url_and_cert(url: &str, cert_fingerprint: Option<&str>) -> Self {
        if url.starts_with("awp://") {
            // AWP sovereign pages — check if AIEONYX CA fingerprint matches
            if let Some(fp) = cert_fingerprint {
                if fp == AIEONYX_CA_FINGERPRINT_SHA256 {
                    return TrustIndicator::AieonxCa;
                }
            }
            return TrustIndicator::SovereignStar;
        }

        if url.starts_with("https://") {
            // HTTPS — check if cert is from AIEONYX CA
            if let Some(fp) = cert_fingerprint {
                if fp == AIEONYX_CA_FINGERPRINT_SHA256 {
                    return TrustIndicator::AieonxCa;
                }
            }
            return TrustIndicator::LegacyHttps;
        }

        if url.starts_with("http://") {
            return TrustIndicator::LegacyHttp;
        }

        TrustIndicator::Unknown
    }

    /// Symbol shown in the URL bar trust indicator.
    pub fn symbol(&self) -> &'static str {
        match self {
            TrustIndicator::Unknown       => "",
            TrustIndicator::LegacyHttps  => "🔒",
            TrustIndicator::LegacyHttp   => "⚠",
            TrustIndicator::SovereignStar => "✶",
            TrustIndicator::AieonxCa     => "✶",
        }
    }

    /// CSS class applied to the trust indicator in the chrome.
    pub fn css_class(&self) -> &'static str {
        match self {
            TrustIndicator::Unknown       => "trust-unknown",
            TrustIndicator::LegacyHttps  => "trust-https",
            TrustIndicator::LegacyHttp   => "trust-http",
            TrustIndicator::SovereignStar => "trust-sovereign",
            TrustIndicator::AieonxCa     => "trust-aieonyx-ca",
        }
    }
}
