<p align="center">
  <img src="assets/onyxia_banner.png" alt="Onyxia — The Sovereign Browser" />
</p>

<p align="center">
  <strong>The Sovereign Browser.</strong><br/>
  Built on Rust. Governed by you.
</p>

<p align="center">
  <a href="https://github.com/aieonyx/onyxia/releases/tag/v1.0.0">
    <img src="https://img.shields.io/badge/release-v1.0.0-00E5FF?style=flat-square" alt="v1.0.0" />
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

**Linux — v1.0.0**

| Format | Size | Notes |
|--------|------|-------|
| [Onyxia_1.0.0_amd64.deb](https://github.com/aieonyx/onyxia/releases/tag/v1.0.0) | 5.1 MB | Debian / Ubuntu / Pop!_OS |
| [Onyxia_1.0.0_amd64.AppImage](https://github.com/aieonyx/onyxia/releases/tag/v1.0.0) | 79 MB | Any x86_64 Linux distro |

macOS and Windows builds are planned for v1.1.

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
|           Onyxia Browser Chrome         |  <- TypeScript / WebKitGTK (160px)
|  Tabs | Nav | ARPi Bar | Trust | Vault  |
+----------------+------------------------+
                 | Tauri IPC (Rust commands)
+----------------v------------------------+
|            Rust Backend                 |
|  Tab Manager | AWP Handler | SSV / STS  |
|  Vault | Session | Legacy | Aegis       |
+----------------+------------------------+
                 |
+----------------v------------------------+
|           EdisonDB  (port 7777)         |  <- Sovereign local database
|  Sessions | Legacy Testament | Threat   |
+-----------------------------------------+
```

### Stack

- **[Tauri v2](https://tauri.app)** — Rust backend, native window, WebKitGTK webview on Linux
- **Rust** — all security-critical logic: vault, session, identity, protocol handling, threat detection
- **TypeScript** — browser chrome UI only; no access to sensitive state
- **[EdisonDB](https://github.com/aieonyx/edisondb)** — sovereign embedded database for sessions, credentials, and legacy testament
- **[AXON](https://github.com/aieonyx/axon)** — sovereign systems language; AWP verifier FFI integration in C13

### Security model

- Trust state is computed exclusively in the Rust process. The frontend receives rendered state only.
- Credentials never cross the IPC boundary in plaintext.
- The URL bar trust indicator (`*` sovereign / `lock` HTTPS / `!` HTTP) is set by the backend — never by page logic.
- Vault is locked by default. No sensitive data is served in locked state.
- All sovereign pages (`awp://`) are handled natively in Rust; no external network request is made.

### Protocol support

| Protocol | Status | Description |
|----------|--------|-------------|
| `https://` | Live | Standard TLS, legacy connection mode |
| `awp://` | Live | AXON Web Protocol — sovereign internal pages |
| AWP mesh routing | Planned C13+ | Peer-to-peer sovereign mesh via AXON verifier |

---

## Features

### ARPi Verification Bar
Every connection is evaluated against the five-layer ARPi stack:
`L1 Schema` -> `L2 Identity` -> `L3 Auth` -> `L4 Scope` -> `L5 Anomaly`

Legacy HTTPS connections show a legacy indicator. AWP connections show live layer verification status.

### Sovereign Threat Sensor (SSV / STS)
- Tracker detection against a curated blocklist
- SSV typosquat detection — flags domain lookalikes
- Mixed content detection
- Crypto-drainer domain allowlist

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
| C1 | Tauri v2 shell — browser chrome, multi-tab, navigation | Complete |
| C2 | AWP protocol handler (`awp://` sovereign pages) | Complete |
| C3 | AXON-Client header emission | Complete |
| C4 | ARPi status bar, protocol switcher, tab URL restore | Complete |
| C5 | Trust indicators, window controls (`decorations=false`) | Complete |
| C6 | Session persistence — EdisonDB tab save/restore | Complete |
| C7 | Sovereign Threat Sensor — tracker / SSV / STS / mixed content | Complete |
| C8 | STS content blocking integration | Complete |
| C9 | Password manager — vault panel, save banner, master password | Complete |
| C10 | Digital Legacy — testament, heartbeat, EdisonDB integration | Complete |
| C11 | Aegis Sovereign Threat Intel UI (`awp://aegis`) | Complete |
| C12 | Production build — v1.0.0 `.deb` + `.AppImage`, binary fixes | Complete |
| C13 | AXON Integration — AWP verifier FFI bridge | In progress |
| C14 | AIEONYX CA pre-installation | Planned |
| C15 | Sovereign renderer (AXON Display Protocol) | Planned v1.1 |

---

## Building from source

### Requirements — Linux (Ubuntu / Debian / Pop!_OS)

```bash
sudo apt install -y \
  libwebkit2gtk-4.1-dev libssl-dev libgtk-3-dev \
  libayatana-appindicator3-dev librsvg2-dev \
  libsoup-3.0-dev libjavascriptcoregtk-4.1-dev \
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
| Linux x86_64 (Ubuntu 22.04+, Pop!_OS, Debian) | v1.0.0 current |
| macOS | Planned v1.1 |
| Windows | Planned v1.1 |

---

## Part of the AIEONYX platform

| Component | Role | Repository |
|-----------|------|------------|
| **AXON** | Sovereign systems language and compiler | [github.com/aieonyx/axon](https://github.com/aieonyx/axon) |
| **EdisonDB** | Sovereign embedded database | [github.com/aieonyx/edisondb](https://github.com/aieonyx/edisondb) |
| **Onyxia** | Sovereign browser | This repository |
| **BASTION OS** | Sovereign node OS (seL4 + Rust) | Planned |
| **aixOs** | Sovereign desktop OS | Planned |

The S4+i framework governs all design decisions: **Security -> Sovereignty -> Simplicity -> Speed -> +Intelligence**.

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
