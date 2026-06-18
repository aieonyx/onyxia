# Onyxia — NLNet NGI Zero Exhibit

**Project:** Onyxia — The Sovereign Browser
**Exhibit target:** NLNet NGI Zero Commons Fund / NGI Fediversity
**Deadline:** August 1, 2026
**Repository:** https://github.com/aieonyx/onyxia
**Author:** Edison Lepiten / AIEONYX

---

## Exhibit Narrative

> *"EdisonDB stores. AXON enforces. Onyxia surfaces."*

Onyxia is a desktop web browser built from first principles on the question:
what does a browser look like when sovereignty is the architecture, not a feature?

Most browsers treat the user's data, identity, and browsing behaviour as raw
material for profiling and monetisation. Onyxia inverts this: the user's device
is the trust boundary, the Rust backend is the security enforcer, and the
browser chrome is a sovereign surface — not a data collection point.

Onyxia is not a fork. It is a ground-up Tauri v2 + Rust implementation with:
- A custom AWP sovereign protocol (awp://) handled natively in Rust
- A five-layer ARPi verification bar driven by the AXON language FFI
- A local sovereign vault, threat sensor, digital legacy, and CA trust system
- Zero telemetry, zero cloud sync, zero vendor account

---

## Technical Architecture
+-----------------------------------------+

|           Onyxia Browser Chrome         |  TypeScript / WebKitGTK (160px)

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

|     AXON (axon_awp_ffi crate)           |  Sovereign language FFI

|  verify_url() -> AxonVerifyResult       |

+----------------+------------------------+

|

+----------------v------------------------+

|           EdisonDB  (port 7777)         |  Sovereign local database

|  Sessions | Legacy | Credentials        |

+-----------------------------------------+
Trust boundary invariant: all security state computed in Rust. Frontend renders only.

---

## Track C — Completed Milestones (C1-C15)

| Milestone | Description | Status |
|-----------|-------------|--------|
| C1 | Tauri v2 shell — browser chrome, multi-tab, navigation | Complete |
| C2 | AWP protocol handler (awp:// sovereign pages) | Complete |
| C3 | AXON-Client header emission | Complete |
| C4 | ARPi status bar, protocol switcher, tab URL restore | Complete |
| C5 | Trust indicators, window controls (decorations=false) | Complete |
| C6 | Session persistence — EdisonDB tab save/restore | Complete |
| C7 | Sovereign Threat Sensor — tracker / SSV / STS / mixed content | Complete |
| C8 | STS content blocking integration | Complete |
| C9 | Password manager — vault panel, master password | Complete |
| C10 | Digital Legacy — testament, heartbeat, EdisonDB | Complete |
| C11 | Aegis Sovereign Threat Intel UI (awp://aegis) | Complete |
| C12 | Production build — v1.0.0 .deb + .AppImage, binary fixes | Complete |
| C13 | AXON Integration — AWP verifier FFI bridge | Complete |
| C14 | AIEONYX CA pre-installation (Ed25519, embedded) | Complete |
| C15 | NLNet exhibit — arXiv paper, evidence package | Complete |

---

## CS Contributions

### TERM-047 — Sovereign FFI Boundary Pattern
A browser security enforcement layer calling into a sovereign systems language
via a C-compatible FFI boundary, with deterministic stub verification and a
feature-flag drop-in point for live cryptographic verification.

### TERM-048 — Production WebKitGTK CSS Variable Resolution Failure
WebKitGTK in a Tauri v2 child webview does not reliably resolve CSS custom
properties (var()) during initial layout paint. Explicit hex fallbacks required.
Discovered and fixed during C12 production binary debugging. (KNOWN-BUG-009)

### TERM-049 — Sovereign CA Embedding Pattern
A browser binary embedding its own Root CA via Rust include_str!() at compile
time, with runtime config-dir write for system trust import, without
system-level CA bundle modification.

---

## EdisonDB Integration Evidence

EdisonDB validated as live data layer for Onyxia v1.0.0:

| Feature | EdisonDB Tier | Operation |
|---------|--------------|-----------|
| Session persistence | PERSONAL | Write on tab close, read on launch |
| Digital Legacy testament | PERSONAL | Save/load via Tauri IPC |
| Sovereign Vault | CRITICAL | Credential store |
| Aegis threat log | NOISE | Session event log |

REST API (port 7777) confirmed stable under browser workload.
Graceful degradation confirmed — Onyxia operates without EdisonDB.

---

## NLNet Alignment

| NGI Zero Criterion | Onyxia Evidence |
|--------------------|-----------------|
| Open source | Apache 2.0, all code public |
| Internet freedom / privacy | Zero telemetry, local vault, sovereign threat sensor |
| Decentralisation | AWP mesh protocol, no cloud dependency |
| EU alignment | Prague-based, EU AI Act aware, GDPR-compatible architecture |
| Technical novelty | Sovereign FFI bridge, ARPi 5-layer model, CA embedding pattern |
| Reproducible build | cargo tauri build from public source |

Recommended call: NGI Zero Commons Fund or NGI Fediversity

---

## Contact

Edison Lepiten / AIEONYX
aieonyx.eu@gmail.com
https://github.com/aieonyx
Prague, Czech Republic

Copyright (c) 2026 Edison Lepiten / AIEONYX — Apache License 2.0
