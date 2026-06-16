// Copyright (c) 2026 Edison Lepiten / AIEONYX
// SPDX-License-Identifier: Apache-2.0
//
// C7-B: Sovereign Site Verification (SSV)
// Detects typosquatting, phishing, and unauthorized crypto sites.
// INVARIANT: all verification runs in Rust main process only.
// INVARIANT: no external network calls — fully offline, sovereign.

use crate::browser::sts::{extract_host, ThreatEvent};

/// Known-good domains — canonical legitimate sites
const KNOWN_GOOD: &[(&str, &str)] = &[
    ("paypal.com", "finance"),
    ("chase.com", "finance"),
    ("bankofamerica.com", "finance"),
    ("wellsfargo.com", "finance"),
    ("citibank.com", "finance"),
    ("hsbc.com", "finance"),
    ("barclays.co.uk", "finance"),
    ("revolut.com", "finance"),
    ("wise.com", "finance"),
    ("binance.com", "crypto"),
    ("coinbase.com", "crypto"),
    ("kraken.com", "crypto"),
    ("ledger.com", "crypto"),
    ("trezor.io", "crypto"),
    ("metamask.io", "crypto"),
    ("uniswap.org", "crypto"),
    ("ethereum.org", "crypto"),
    ("bitcoin.org", "crypto"),
    ("irs.gov", "government"),
    ("gov.uk", "government"),
    ("europa.eu", "government"),
];

const CRYPTO_KEYWORDS: &[&str] = &[
    "binance", "coinbase", "kraken", "bitcoin", "ethereum", "crypto",
    "wallet", "defi", "nft", "web3", "metamask", "ledger", "trezor",
    "uniswap", "blockchain", "token", "altcoin", "exchange",
];

fn levenshtein(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let m = a.len();
    let n = b.len();
    let mut dp = vec![vec![0usize; n + 1]; m + 1];
    for i in 0..=m { dp[i][0] = i; }
    for j in 0..=n { dp[0][j] = j; }
    for i in 1..=m {
        for j in 1..=n {
            dp[i][j] = if a[i-1] == b[j-1] {
                dp[i-1][j-1]
            } else {
                1 + dp[i-1][j-1].min(dp[i-1][j]).min(dp[i][j-1])
            };
        }
    }
    dp[m][n]
}

fn normalize_domain(domain: &str) -> String {
    domain.trim_start_matches("www.").to_lowercase()
}

fn as_crypto_keywords(host: &str) -> bool {
    let h = host.to_lowercase();
    CRYPTO_KEYWORDS.iter().any(|&kw| h.contains(kw))
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SsvVerdict {
    pub threat: ThreatEvent,
    pub suggested_real: Option<String>,
    pub block: bool,
}

pub fn verify_site(url: &str) -> Option<SsvVerdict> {
    let host = extract_host(url)?;
    let norm = normalize_domain(&host);

    if as_crypto_keywords(&norm) {
        let ok = KNOWN_GOOD.iter().any(|(g, c)| *c == "crypto" && normalize_domain(g) == norm);
        if !ok {
            let sug = KNOWN_GOOD.iter().filter(|(_, c)| *c == "crypto")
                .min_by_key(|(g, _)| levenshtein(&norm, &normalize_domain(g)))
                .map(|(g, _)| g.to_string());
            return Some(SsvVerdict {
                threat: ThreatEvent { kind: "unauthorized_crypto_site".to_string(), domain: host, url: url.to_string() },
                suggested_real: sug, block: true,
            });
        }
    }

    for (gd, cat) in KNOWN_GOOD {
        let gn = normalize_domain(gd);
        if gn == norm { continue; }
        let d = levenshtein(&norm, &gn);
        if d <= 2 && d > 0 {
            return Some(SsvVerdict {
                threat: ThreatEvent { kind: format!("typosquat_{}", cat), domain: host, url: url.to_string() },
                suggested_real: Some(gd.to_string()), block: true,
            });
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_legit_crypto() {
        assert!(verify_site("https://binance.com/trade").is_none());
        assert!(verify_site("https://www.coinbase.com/").is_none());
    }
    #[test]
    fn test_fake_crypto() {
        assert!(verify_site("https://binance-pro.net/").is_some());
        assert!(verify_site("https://crypto-wallet-secure.com/").is_some());
    }
    #[test]
    fn test_typosquat() {
        assert!(verify_site("https://paypa1.com/login").is_some());
        assert!(verify_site("https://paypal.com/signin").is_none());
    }
    #[test]
    fn test_levenshtein() {
        assert_eq!(levenshtein("paypal", "paypa1"), 1);
        assert_eq!(levenshtein("binance", "binance"), 0);
    }
}
