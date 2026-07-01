<p align="center">
  <img src="assets/onyxia_banner.png" alt="Onyxia — The Sovereign Browser" />
</p>

<p align="center">
  <strong>The Sovereign Browser.</strong><br/>
  Built on Rust. Governed by you.
</p>

<p align="center">
  <a href="https://github.com/aieonyx/onyxia/releases/tag/v1.1.0">
    <img src="https://img.shields.io/badge/release-v1.1.0-00E5FF?style=flat-square" alt="v1.1.0" />
  </a>
  <img src="https://img.shields.io/badge/platform-Linux-7B2FBE?style=flat-square" alt="Linux" />
  <img src="https://img.shields.io/badge/license-Apache--2.0-00C853?style=flat-square" alt="Apache 2.0" />
  <img src="https://img.shields.io/badge/built_with-Rust-orange?style=flat-square" alt="Rust" />
  <img src="https://img.shields.io/badge/part_of-AIEONYX-00E5FF?style=flat-square" alt="AIEONYX" />
</p>

---

Onyxia is a desktop web browser for people who choose not to be tracked, profiled, or monetised. It is part of the [AIEONYX](https://github.com/aieonyx) sovereign digital infrastructure platform — a stack built on the principle that every person has the right to full control of their digital existence.

> *"EdisonDB stores. AXON enforces. Onyxia surfaces."*

---

## Download

**Linux — v1.1.0**

| Format | Size | Notes |
|--------|------|-------|
| [Onyxia_1.1.0_amd64.deb](https://github.com/aieonyx/onyxia/releases/tag/v1.1.0) | ~5 MB | Debian / Ubuntu / Pop!_OS |
| [Onyxia_1.1.0_amd64.AppImage](https://github.com/aieonyx/onyxia/releases/tag/v1.1.0) | ~78 MB | Any x86_64 Linux distro |

macOS and Windows builds are planned for a future release.

---

## What makes Onyxia different

Most browsers treat your browsing data as something to collect and exploit. Onyxia starts from a different premise: your data belongs to you, it stays on your device, and it only leaves when you explicitly decide.

| Feature | Legacy Browser | Onyxia |
|---------|---------------|--------|
| Password storage | Cloud-synced, vendor-controlled | Local sovereign vault, your key |
| Autofill | Silent, from remote servers | Explicit confirmation required |
| Tracker blocking | Optional plugin | Built-in Sovereign Threat Sensor |
| Telemetry | On by default | Zero — no telemetry, no analytics |
| Certificate trust | Vendor decides | You decide, AIEONYX CA support |
| Identity | Cookies, fingerprinting | Sovereign identity via ARPi protocol |
| Threat intelligence | None or cloud-sourced | Local Aegis engine, no cloud dependency |
| Data after death | Scattered, inaccessible | Sovereign Digital Legacy — your testament |

---

## Architecture

Onyxia is built with a strict trust boundary: **the Rust backend computes all security state; the TypeScript frontend only renders what it is told.**

```
+-----------------------------------------+
|           Onyxia Browser Chrome         |  <- TypeScript (160px)
|  Tabs | Nav | ARPi Bar | Trust | Vault  |
+----------------+------------------------+
                 | Tauri IPC (Rust commands)
+----------------v------------------------+
|            Rust Backend                 |
|  Tab Manager | AWP Handler | SSV / STS  |
|  Vault | Session | Legacy | Aegis       |
+----------------+------------------------+
                 |
       +---------+----------+
       |                    |
+------v-------+    +-------v---------+
|    HANIEL    |    | EdisonDB        |
|  Sovereign   |    | Sovereign DB    |
|  Rendering   |    | Sessions/Vault  |
|  Pipeline    |    | Legacy/Aegis    |
|  HERALD      |    +-----------------+
|  PRISM       |
|  CANVAS      |
+--------------+
       |
+------v-------+
| axon_awp     |
| AWP protocol |
| 11 categories|
| 249 regions  |
+--------------+
```

Trust boundary invariant: all security state computed in Rust. Frontend renders only.

### Stack

- **[Tauri v2](https://tauri.app)** — Rust backend, native window, IPC bridge
- **HANIEL** — sovereign rendering engine (HERALD fetch+threat-gate → PRISM parse+layout → CANVAS rasterize); replaces third-party rendering engine dependency
- **Rust** — all security-critical logic: vault, session, identity, protocol handling, threat detection
- **TypeScript** — browser chrome UI only; no access to sensitive state
- **[axon_awp](https://github.com/aieonyx/AXON)** — sovereign AWP protocol (11 categories, ISO 3166-1 regions, FFI exports)
- **[EdisonDB](https://github.com/aieonyx/edisondb)** — sovereign embedded database for sessions, credentials, and legacy testament
- **[AXON](https://github.com/aieonyx/AXON)** — sovereign systems language; AWP verifier FFI bridge, Ed25519 Root CA

### Security model

- Trust state is computed exclusively in the Rust process. The frontend receives rendered state only.
- Credentials never cross the IPC boundary in plaintext.
- The URL bar trust indicator (`*` sovereign / `lock` HTTPS / `!` HTTP) is set by the backend — never by page logic.
- Vault is locked by default. No sensitive data is served in locked state.
- All sovereign pages (`awp://`) are handled natively in Rust via axon_awp; no external network request is made.
- AIEONYX Root CA embedded at compile time via `include_str!()`.

### Protocol support

| Protocol | Status | Description |
|----------|--------|-------------|
| `https://` | Live | Standard TLS, legacy web — rendered by HANIEL pipeline |
| `awp://` | Live | AXON Web Protocol — sovereign internal pages |
| AWP category routing | Live (axon_awp P66) | 11 categories, regional routing, global fallback |
| AWP mesh routing | Planned | Peer-to-peer sovereign mesh |

---

## Features

### HANIEL Sovereign Rendering Engine
Onyxia v1.1.0 ships with HANIEL — a sovereign rendering pipeline written entirely in Rust. HANIEL owns every stage: fetch, threat-gate, parse, layout, rasterize, encode. No third-party rendering engine dependency.

### ARPi Verification Bar
Every connection is evaluated against the five-layer ARPi stack:
`L1 Schema` → `L2 Identity` → `L3 Auth` → `L4 Scope` → `L5 Anomaly`

Legacy HTTPS connections show a legacy indicator. AWP connections show live layer verification status.

### Sovereign Threat Sensor (SSV / STS)
- Tracker detection — 29 known domains, Levenshtein typosquat detection
- Crypto-drainer domain blocking
- Mixed content detection
- All threat decisions made locally; no cloud lookup

### Aegis Threat Intel (`awp://aegis`)
Local threat intelligence dashboard. No cloud dependency. Surfaces SSV/STS events, threat patterns, and anomaly scores from the current session.

### Sovereign Vault
Local password manager. Master password protected. Credentials stored in EdisonDB Critical tier. Explicit fill confirmation — no silent autofill.

### Digital Legacy (`awp://legacy`)
Sovereign testament for your digital life. Configure an inactivity trigger, designate a legacy holder, and specify what happens to each data tier (Critical / Personal / Noise) when the heartbeat stops.

### Session Persistence
Tabs and navigation state are saved to EdisonDB and restored on next launch. No cloud sync, no vendor account required.

---

## Track C — Build Milestones

| Milestone | Description | Status |
|-----------|-------------|--------|
| C1 | Tauri v2 shell — browser chrome, multi-tab, navigation | ✅ Complete |
| C2 | AWP protocol handler (`awp://` sovereign pages) | ✅ Complete |
| C3 | AXON-Client header emission | ✅ Complete |
| C4 | ARPi status bar, protocol switcher, tab URL restore | ✅ Complete |
| C5 | Trust indicators, window controls (`decorations=false`) | ✅ Complete |
| C6 | Session persistence — EdisonDB tab save/restore | ✅ Complete |
| C7 | Sovereign Threat Sensor — tracker / SSV / STS / mixed content | ✅ Complete |
| C8 | STS content blocking integration | ✅ Complete |
| C9 | Password manager — vault panel, save banner, master password | ✅ Complete |
| C10 | Digital Legacy — testament, heartbeat, EdisonDB integration | ✅ Complete |
| C11 | Aegis Sovereign Threat Intel UI (`awp://aegis`) | ✅ Complete |
| C12 | Production build — v1.0.0 `.deb` + `.AppImage` | ✅ Complete |
| C13 | AXON Integration — AWP verifier FFI bridge (`axon_awp_ffi`) | ✅ Complete |
| C14 | AIEONYX CA pre-installation (Ed25519, embedded) | ✅ Complete |
| C15 | NLNet exhibit — arXiv paper, evidence package | ✅ Complete |
| C16 | HANIEL sovereign rendering engine — all stages (HE-1–HE-15) | ✅ Complete v1.1.0 |
| C17 | AWP mesh routing | 🔵 Planned |

---

## Building from source

### Requirements — Linux (Ubuntu / Debian / Pop!_OS)

```bash
sudo apt install -y \
  libssl-dev libayatana-appindicator3-dev librsvg2-dev \
  libdbus-1-dev libsecret-1-dev pkg-config \
  build-essential curl
```

Install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

Install Tauri CLI:
```bash
cargo install tauri-cli --version "^2.0"
```

### Clone and build

```bash
git clone https://github.com/aieonyx/onyxia.git
cd onyxia
npm install
cargo tauri build
```

Artifacts are written to `target/release/bundle/`.

### Development mode

```bash
cargo tauri dev
```

> **Note:** EdisonDB is an optional dependency. If not running, Onyxia degrades gracefully — session persistence and Digital Legacy are disabled with a status warning.

---

## Platform support

| Platform | Status |
|----------|--------|
| Linux x86_64 (Ubuntu 22.04+, Pop!_OS, Debian) | ✅ v1.1.0 current |
| macOS | 🔵 Planned |
| Windows | 🔵 Planned |

---

## Part of the AIEONYX sovereign stack

| Component | Role | Status |
|-----------|------|--------|
| **[AXON](https://github.com/aieonyx/AXON)** | Sovereign compiler, AWP protocol, LSP, package registry | ✅ 1,606+ tests |
| **[EdisonDB](https://github.com/aieonyx/edisondb)** | Sovereign database — WAL, MVCC, RBAC, compliance | ✅ Phase 3 complete |
| **[Onyxia](https://github.com/aieonyx/onyxia)** | Sovereign browser | ✅ v1.1.0 |
| **[BASTION](https://github.com/aieonyx/bastion)** | Sovereign node OS bootstrap — Ed25519 verifier, seL4 | ✅ v0.2.0 |
| **aixOs** | Sovereign desktop OS | 🔵 Planned |

The S4+i framework governs all design decisions: **Security → Sovereignty → Simplicity → Speed → +Intelligence**.

---

## NLNet NGI Zero

Onyxia is submitted to **NLNet NGI Zero Commons Fund / NGI Fediversity** (deadline: August 1, 2026).

Evidence package: [EXHIBIT.md](./EXHIBIT.md) · [STACK_SUMMARY.md](./STACK_SUMMARY.md)

---

## License

Copyright (c) 2026 Edison Lepiten / AIEONYX  
Licensed under the [Apache License, Version 2.0](LICENSE).

---

## Contributing

Onyxia is in active development toward NLNet NGI Zero funding.  
Issues and discussion are welcome. Pull requests are reviewed manually — please open an issue first.

---

> *"We are not users. We are not accounts. We are not products. We are people."*
