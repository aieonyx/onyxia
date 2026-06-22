# AIEONYX Sovereign Stack — Technical Summary
<!-- Copyright (c) 2026 Edison Lepiten / AIEONYX -->
<!-- Companion document for arXiv submission and NLNet evidence package -->

## Overview

AIEONYX is a sovereign digital infrastructure stack built by Edison Lepiten
(Prague, Czech Republic) as a solo engineering project from April–June 2026.
The stack addresses a single architectural question: can a complete computing
environment — compiler, database, browser, and node OS — be built with zero
external trust dependencies and zero vendor lock-in?

The answer demonstrated here is yes, with 1,606+ passing tests across four
public repositories and zero test failures at time of submission.

## Stack Components

### 1. AXON / AXONYX — Sovereign Systems Language
- Self-hosting Rust compiler targeting x86_64, aarch64, and seL4
- Capability-flow type system: @constant_time, @no_alloc, CCP profiles
- Vulkan GPU backend (AMD RADV RENOIR), WASM JIT (sovereign x86_64, no LLVM)
- Transformer attention in pure .ax source (P63.1)
- LSP 3.17 language server, sovereign package registry
- 1,606+ workspace tests, 0 failures

### 2. EdisonDB — Sovereign AI-Native Database
- WAL + MVCC storage (fjall TxKeyspace)
- Inverted Admin Model: owner-first, three data tiers (Critical/Personal/Noise)
- gRPC transport, HNSW vector index, sovereign offline embeddings
- ARPi 78-byte provenance header on every response
- GDPR Art.17 right-to-erasure, compliance reporting
- Phase 3 complete: 178+ tests

### 3. BASTION — Sovereign Node OS Bootstrap
- Ed25519 binary verification (axon_crypto P57.1 curve math)
- Seven-step acceptance gate; DevMode binaries unconditionally rejected
- Node Commissioning Ceremony: keypair sealed in Policy PD
- Nonce replay protection, signer delegation with expiry
- 40 tests across v0.1 + v0.2

### 4. Onyxia — Sovereign Browser
- Tauri v2 + Rust + WebKitGTK; not a Chromium fork
- AWP sovereign protocol (awp://), five-layer ARPi verification bar
- Sovereign Threat Sensor (29 tracker domains), SSV typosquat detection
- Digital Legacy, Password Vault, Aegis Collective dashboard
- Production: 5.1 MB .deb, 79 MB .AppImage

## Novel CS Contributions (TERM-047 through TERM-055)

1. Sovereign FFI Boundary Pattern (TERM-047)
2. Production WebKitGTK CSS var() resolution failure (TERM-048)
3. Sovereign CA Embedding Pattern (TERM-049)
4. Sovereign AWP Protocol — two-tier naming grammar (TERM-050)
5. ARPi Provenance Header — 78-byte fixed wire format (TERM-051)
6. Inverted Admin Model + RBAC (TERM-052)
7. Sovereign Hash Projection Embedding (TERM-053)
8. BASTION Binary Verification Gate (TERM-054)
9. Sovereign Package Manifest (.axpkg) (TERM-055)

## Key Invariants

- Owner always bypasses policy (EdisonDB)
- DevMode binaries always rejected, no override (BASTION)
- Private key never leaves Policy PD (BASTION)
- All security state computed in Rust, frontend renders only (Onyxia)
- Sovereign offline: zero network required for core operations

## arXiv Submissions

- Slot 7680982: AXON sovereign systems language (cs.AR)
- Onyxia browser paper: cs.CR / cs.SE (ARXIV_ABSTRACT.md)
- EdisonDB paper: cs.DB (in preparation)

## License

All components: Apache 2.0
Copyright (c) 2026 Edison Lepiten / AIEONYX
