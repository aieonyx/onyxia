# Onyxia — NLNet NGI Zero Exhibit

**Project:** Onyxia — The Sovereign Browser
**Exhibit target:** NLNet NGI Zero Commons Fund / NGI Fediversity
**Deadline:** August 1, 2026
**Repository:** https://github.com/aieonyx/onyxia
**Author:** Edison Lepiten / AIEONYX
**Updated:** June 2026 — reflects full AIEONYX sovereign stack

---

## Exhibit Narrative

> *"EdisonDB stores. AXON enforces. Onyxia surfaces."*

Onyxia is a desktop web browser built from first principles on the question:
what does a browser look like when sovereignty is the architecture, not a feature?

Most browsers treat the user's data, identity, and browsing behaviour as raw
material for profiling and monetisation. Onyxia inverts this: the user's device
is the trust boundary, the Rust backend is the security enforcer, and the
browser chrome is a sovereign surface — not a data collection point.

Onyxia is not a fork. It is a ground-up Tauri v2 + Rust implementation backed
by a complete sovereign infrastructure stack built in parallel:

- **AXON / AXONYX** — sovereign systems programming language (Rust compiler, seL4 target)
- **EdisonDB** — sovereign AI-native multi-model database (WAL, MVCC, gRPC, ARPi)
- **BASTION** — sovereign node OS bootstrap (Ed25519 binary verification, seL4 PDs)
- **axon_awp** — sovereign AWP protocol core (11 categories, ISO 3166-1 regions, router)
- **axon_lsp** — AXONYX language server (LSP 3.17, diagnostics, hover, document sync)
- **axon_registry** — sovereign package registry (SHA-256 integrity, Ed25519 manifest)

---

## Technical Architecture

```
+-----------------------------------------+
|           Onyxia Browser Chrome         |  TypeScript / WebKitGTK
|  Tabs | Nav | ARPi Bar | Trust | Vault  |
+----------------+------------------------+
                 | Tauri IPC (Rust commands)
+----------------v------------------------+
|            Rust Backend                 |
|  AWP Handler | axon_awp_ffi | SSV/STS  |
|  Vault | Session | Legacy | Aegis | CA  |
+----------------+------------------------+
                 |
+----------------v------------------------+
|     axon_awp (sovereign AWP protocol)   |  11 categories, 249 regions
|  parser | router | FFI | C-ABI exports  |
+----------------+------------------------+
                 |
+----------------v------------------------+
|     AXON (axon_awp_ffi crate)           |  Sovereign language FFI
|  verify_url() -> AxonVerifyResult       |
+----------------+------------------------+
                 |
+----------------v------------------------+
|           EdisonDB  (port 7777)         |  Sovereign local database
|  Sessions | Legacy | Credentials        |
|  ARPi headers | Policy engine | Migrate |
+-----------------------------------------+
                 |
+----------------v------------------------+
|           BASTION                       |  Sovereign node OS
|  Ed25519 verifier | Policy PD | Nonce   |
|  Node Commissioning | Dev-mode reject   |
+-----------------------------------------+
Trust boundary invariant: all security state computed in Rust. Frontend renders only.
```

---

## Track C — Completed Milestones (C1–C15)

| Milestone | Description | Status |
|-----------|-------------|--------|
| C1  | Tauri v2 shell — browser chrome, multi-tab, navigation | Complete |
| C2  | AWP protocol handler (awp:// sovereign pages) | Complete |
| C3  | AXON-Client header emission | Complete |
| C4  | ARPi status bar, protocol switcher, tab URL restore | Complete |
| C5  | Trust indicators, window controls (decorations=false) | Complete |
| C6  | Session persistence — EdisonDB tab save/restore | Complete |
| C7  | Sovereign Threat Sensor — tracker / SSV / STS / mixed content | Complete |
| C8  | STS content blocking integration | Complete |
| C9  | Password manager — vault panel, master password | Complete |
| C10 | Digital Legacy — testament, heartbeat, EdisonDB | Complete |
| C11 | Aegis Sovereign Threat Intel UI (awp://aegis) | Complete |
| C12 | Production build — v1.0.0 .deb + .AppImage, binary fixes | Complete |
| C13 | AXON Integration — AWP verifier FFI bridge | Complete |
| C14 | AIEONYX CA pre-installation (Ed25519, embedded) | Complete |
| C15 | NLNet exhibit — arXiv paper, evidence package | Complete |

---

## AIEONYX Sovereign Stack — Current State (June 2026)

### AXON / AXONYX Compiler (github.com/aieonyx/AXON)

| Component | Milestone | Tests | Status |
|-----------|-----------|-------|--------|
| Self-hosting bootstrap | P45–P55 | 109 | ✅ v0.55-bootstrap |
| HANIEL modules (P56–P63) | 8 crates | 1,446 | ✅ |
| Ed25519 full curve math | P57.1 | 93 | ✅ |
| @constant_time codegen | P55.7 | 34 | ✅ |
| Vulkan GPU backend | P58.1 | 30 | ✅ |
| axon_std sync/mem | Tier 1 | — | ✅ |
| axon_sel4 seL4 rewrite | M1–M4 | — | ✅ merged |
| Transformer attention (.ax) | P63.1 | 20 | ✅ |
| ECHO WASM JIT (x86_64) | P61.1 | 20 | ✅ |
| Language Server (LSP 3.17) | P64 | 20 | ✅ |
| Sovereign package registry | P65 | 20 | ✅ |
| AWP protocol core | P66 | 20 | ✅ |
| **Workspace total** | | **1,606+** | **0 failed** |

### EdisonDB (github.com/aieonyx/edisondb)

| Milestone | Deliverable | Tests | Status |
|-----------|-------------|-------|--------|
| P3-M1 | WAL + MVCC (fjall TxKeyspace) | 58 | ✅ |
| P3-M2 | gRPC server (tonic) | — | ✅ |
| P3-M4 | Sovereign offline embeddings | 20 | ✅ |
| P3-M5 | ARPi protocol integration | 20 | ✅ |
| P3-M6 | Access control + policy engine | 20 | ✅ |
| P3-M7 | Migration toolkit (.edm format) | 20 | ✅ |
| P3-M8 | Formal verification hooks | 20 | ✅ |
| P3-M9 | Compliance tooling (GDPR Art.17) | 20 | ✅ |
| **Phase 3 total** | | **178+** | **Phase 3 complete** |

### BASTION (github.com/aieonyx/bastion)

| Version | Deliverable | Tests | Status |
|---------|-------------|-------|--------|
| v0.1.0 | Bootstrap: manifest, verifier, commissioning | 20 | ✅ |
| v0.2.0 | Real Ed25519 via axon_crypto (P57.1) | 20 | ✅ |
| **Hard invariant** | DevMode binaries ALWAYS rejected | — | enforced |

---

## CS Contributions (updated)

### TERM-047 — Sovereign FFI Boundary Pattern
Browser security enforcement via C-ABI FFI into a sovereign systems language,
with deterministic stub verification and feature-flag live crypto drop-in.

### TERM-048 — Production WebKitGTK CSS Variable Resolution Failure
WebKitGTK in Tauri v2 child webviews does not reliably resolve CSS custom
properties during initial layout paint. Hex fallbacks required. (KNOWN-BUG-009)

### TERM-049 — Sovereign CA Embedding Pattern
Browser binary embedding Root CA via Rust include_str!() at compile time,
runtime config-dir write for system trust import, no system CA bundle modification.

### TERM-050 — Sovereign AWP Protocol
Two-tier naming grammar (name.category[.region]) with fixed category registry
(11 categories), ISO 3166-1 alpha-2 regional routing (249 codes), and
regional-first with global fallback dispatch. Zero DNS dependency.

### TERM-051 — ARPi Provenance Header
78-byte fixed wire format carrying DataTier, audit chain hash (SHA-256),
record count, timestamp, and integrity seal — enabling data provenance
verification without trusting the transport layer.

### TERM-052 — Inverted Admin Model + RBAC
Owner-first access control: owner always bypasses policy; Admin role cannot
Grant; Reader/Writer cannot access Critical tier; delegation with expiry;
explicit Deny overrides Allow. Five-role RBAC on three data tiers.

### TERM-053 — Sovereign Hash Projection Embedding
Deterministic text embedding via FNV-1a hash projection into R^128 with
bigram co-occurrence weighting and L2 normalization. Zero network, zero
model files, zero external dependency. Fully reproducible offline.

### TERM-054 — BASTION Binary Verification Gate
Seven-step binary acceptance gate: version → dev-mode hard rejection →
Ed25519 signature → content hash → signer trust → nonce replay → policy.
Dev-mode rejection is unconditional and cannot be overridden.

### TERM-055 — Sovereign Package Manifest (.axpkg)
Ed25519-signed package manifest with SHA-256 content hash, semantic version
ordering, capability declarations, and monotonic nonce replay protection.
Zero crates.io dependency for package verification.

---

## EdisonDB Integration Evidence

| Feature | EdisonDB Tier | Component |
|---------|--------------|-----------|
| Session persistence | PERSONAL | P3-M1 WAL + MVCC |
| Digital Legacy testament | PERSONAL | P3-M1 fjall backend |
| Sovereign Vault | CRITICAL | P3-M6 policy engine |
| Aegis threat log | NOISE | P3-M1 audit log |
| Offline embeddings | — | P3-M4 sovereign embedder |
| ARPi provenance | all tiers | P3-M5 ARPi header |
| GDPR erasure | PERSONAL | P3-M9 compliance |

---

## Production Artifacts

| Artifact | Size | Format |
|----------|------|--------|
| Onyxia v1.0.0 .deb | 5.1 MB | Debian package |
| Onyxia v1.0.0 .AppImage | 79 MB | Portable Linux |
| AXON compiler binary | ~8 MB | ELF x86_64 |
| EdisonDB server binary | ~12 MB | ELF x86_64 |
| BASTION verifier | ~2 MB | ELF x86_64 |

---

## NLNet Alignment

| NGI Zero Criterion | Evidence |
|--------------------|----------|
| Open source | Apache 2.0 — AXON, EdisonDB, Onyxia, BASTION all public |
| Internet freedom / privacy | Zero telemetry, GDPR Art.17 erasure, sovereign vault |
| Decentralisation | AWP mesh protocol, no cloud dependency, BASTION node OS |
| EU alignment | Prague-based, GDPR-compliant architecture, EU AI Act aware |
| Technical novelty | 9 new CS terms (TERM-047–055), 1,606+ tests, sovereign JIT |
| Reproducible build | cargo tauri build / cargo build from public source |
| Interoperability | LSP 3.17, gRPC, C-ABI FFI, AWP protocol open spec |

Recommended call: NGI Zero Commons Fund or NGI Fediversity

---

## Repository Map

| Repo | Purpose | Language | Stars |
|------|---------|----------|-------|
| github.com/aieonyx/AXON | Sovereign compiler + HANIEL | Rust + .ax | — |
| github.com/aieonyx/edisondb | Sovereign database | Rust | — |
| github.com/aieonyx/onyxia | Sovereign browser | Rust + TypeScript | — |
| github.com/aieonyx/bastion | Sovereign node OS | Rust | — |

---

## Contact

Edison Lepiten / AIEONYX
aieonyx.eu@gmail.com
https://github.com/aieonyx
Prague, Czech Republic

Copyright (c) 2026 Edison Lepiten / AIEONYX — Apache License 2.0
