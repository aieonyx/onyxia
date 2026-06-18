# arXiv Abstract — Onyxia Sovereign Browser

**Title:** Onyxia: A Sovereign Browser Architecture with FFI-Driven Protocol
Verification and Embedded CA Trust

**Authors:** Edison Lepiten (AIEONYX, Prague, Czech Republic)

**Category:** cs.CR (Cryptography and Security) / cs.SE (Software Engineering)

**Keywords:** sovereign computing, browser security, Rust, WebKitGTK, Tauri,
FFI, protocol verification, certificate authority, digital sovereignty

---

## Abstract

We present Onyxia, a desktop web browser designed around a strict sovereignty
architecture: all security state is computed exclusively in a Rust backend
process, while the browser chrome renders only what the backend authorises.
Onyxia introduces three architectural contributions.

First, the AXON Receptor Protocol Interface (ARPi) — a five-layer connection
verification model (Schema, Identity, Mutual Auth, Scope, Anomaly) evaluated on
every navigation via a C-compatible FFI boundary into the AXON sovereign systems
language (axon_awp_ffi crate). This establishes a pattern for browsers whose
trust enforcement is delegated to a formally-verified language runtime rather
than browser-internal heuristics.

Second, the AWP (AXON Web Protocol) — a sovereign URI scheme (awp://) whose
pages are served natively from Rust without any network request, providing a
foundation for peer-to-peer sovereign mesh routing independent of the DNS and
Certificate Authority systems.

Third, a sovereign CA embedding pattern: the AIEONYX Root CA (Ed25519, 10-year
validity) is compiled into the binary via Rust include_str!() and written to the
user config directory at launch, enabling AIEONYX-signed connections to be
identified without system-level CA bundle modification.

Onyxia is built on Tauri v2 with WebKitGTK on Linux. We document a previously
unreported production defect in WebKitGTK child webviews: CSS custom properties
(var()) are not reliably resolved during initial layout paint when the webview
is created via window.add_child(), requiring explicit hex fallbacks on all
structural background properties (KNOWN-BUG-009).

The browser integrates EdisonDB for session persistence, digital legacy, and
local credential storage, demonstrating a complete sovereign data stack:
EdisonDB stores, AXON enforces, Onyxia surfaces.

Source: https://github.com/aieonyx/onyxia
License: Apache 2.0
Platform: Linux x86_64 (v1.0.0); macOS/Windows planned v1.1

---

## Submission notes

- Related: AXON sovereign systems language (cs.AR, slot 7680982)
- Related: EdisonDB sovereign database (cs.DB, in preparation)
- All three form the AIEONYX sovereign digital infrastructure platform
