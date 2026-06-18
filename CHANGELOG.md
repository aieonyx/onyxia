## [v1.1.0-rc1] — 2026-06-18

### C13 — AXON Integration (AWP Verifier FFI Bridge)
- New crate: `axon_awp_ffi` in AXON workspace
- Real `extern "C"` boundary: `axon_verify_awp_packet(packet, len) -> AxonVerifyResult`
- Five-layer ARPi result driven by AXON FFI on every navigation (not hardcoded)
- `arpi_state.rs`: `ArpiStatus::from_axon_verify()` — live FFI call per URL
- `awp_handler.rs`: `X-AXON-Verified` response header; `AXON-Client: onyxia/1.0.0`
- AXON-STUB-001: deterministic stub active; `axon-live` feature flag for P45 mesh verifier
- 5/5 tests passing in `axon_awp_ffi`

### C14 — AIEONYX CA Pre-installation
- AIEONYX Root CA: Ed25519, 10-year validity (2026-2036), EU/Prague
- SHA-256: 64:6D:99:AD:46:C2:50:A8:83:A1:95:C1:36:25:0B:1E:47:EC:5F:BE:CB:90:A2:C4:C0:28:DC:8F:8C:B7:8A:89
- CA PEM embedded in binary via include_str!() — zero runtime dependency
- Written to ~/.config/onyxia/aieonyx_ca.pem at launch for system trust import
- TrustIndicator::AieonxCa active on cert fingerprint match
- AUDIT-002: WebKitGTK TLS DB registration deferred to v1.1

### C15 — NLNet Exhibit
- arXiv paper draft: cs.CR/cs.SE — Onyxia sovereign browser architecture
- EXHIBIT.md: NLNet NGI Zero evidence package
- README milestone table updated C1-C15
- Tag: v1.1.0-rc1

---

