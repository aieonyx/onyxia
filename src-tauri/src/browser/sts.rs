// Copyright (c) 2026 Edison Lepiten / AIEONYX
// SPDX-License-Identifier: Apache-2.0
//
// C7: Sovereign Threat Sensor (STS)
// Detects tracker domains, mixed content, and suspicious patterns.
// INVARIANT: all threat analysis runs in Rust main process only.
// INVARIANT: frontend never computes threat state.

/// Known tracker and surveillance domains — STS blocklist v1
const TRACKER_DOMAINS: &[&str] = &[
    // Advertising networks
    "doubleclick.net",
    "googlesyndication.com",
    "googleadservices.com",
    "adnxs.com",
    "advertising.com",
    "amazon-adsystem.com",
    "adsafeprotected.com",
    "openx.net",
    "rubiconproject.com",
    "pubmatic.com",
    "criteo.com",
    "criteo.net",
    "outbrain.com",
    "taboola.com",
    "moatads.com",
    // Analytics and tracking
    "google-analytics.com",
    "googletagmanager.com",
    "googletagservices.com",
    "analytics.google.com",
    "hotjar.com",
    "mixpanel.com",
    "segment.com",
    "segment.io",
    "heap.io",
    "fullstory.com",
    "mouseflow.com",
    "logrocket.com",
    "clarity.ms",
    // Social trackers
    "facebook.net",
    "connect.facebook.net",
    "platform.twitter.com",
    "platform.linkedin.com",
    "snap.licdn.com",
    // Fingerprinting
    "fingerprintjs.com",
    "fingerprint.com",
    // Data brokers
    "scorecardresearch.com",
    "quantserve.com",
    "comscore.com",
    "nielsen.com",
];

#[derive(Debug, Clone, serde::Serialize)]
pub struct ThreatEvent {
    pub kind: String,
    pub domain: String,
    pub url: String,
}

/// Extract host from a URL
pub fn extract_host(url: &str) -> Option<String> {
    let url = url
        .trim_start_matches("https://")
        .trim_start_matches("http://");
    let host = url.split('/').next()?;
    let host = host.split('?').next()?;
    Some(host.to_lowercase())
}

/// Check if a URL matches a known tracker domain
pub fn is_tracker(url: &str) -> Option<ThreatEvent> {
    let host = extract_host(url)?;
    for &tracker in TRACKER_DOMAINS {
        if host == tracker || host.ends_with(&format!(".{}", tracker)) {
            return Some(ThreatEvent {
                kind: "tracker_domain".to_string(),
                domain: tracker.to_string(),
                url: url.to_string(),
            });
        }
    }
    None
}

/// Check for mixed content (HTTP resource on HTTPS page)
pub fn is_mixed_content(page_url: &str, resource_url: &str) -> Option<ThreatEvent> {
    if page_url.starts_with("https://") && resource_url.starts_with("http://") {
        let host = extract_host(resource_url).unwrap_or_default();
        return Some(ThreatEvent {
            kind: "mixed_content".to_string(),
            domain: host,
            url: resource_url.to_string(),
        });
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracker_detection() {
        assert!(is_tracker("https://www.google-analytics.com/collect").is_some());
        assert!(is_tracker("https://static.doubleclick.net/ads").is_some());
        assert!(is_tracker("https://www.github.com/page").is_none());
        assert!(is_tracker("https://servo.org/").is_none());
    }

    #[test]
    fn test_mixed_content() {
        assert!(is_mixed_content(
            "https://example.com",
            "http://cdn.example.com/script.js"
        ).is_some());
        assert!(is_mixed_content(
            "https://example.com",
            "https://cdn.example.com/script.js"
        ).is_none());
    }

    #[test]
    fn test_extract_host() {
        assert_eq!(extract_host("https://www.google.com/path"), Some("www.google.com".into()));
        assert_eq!(extract_host("https://doubleclick.net/"), Some("doubleclick.net".into()));
    }
}
