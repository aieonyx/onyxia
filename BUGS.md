# Onyxia Bug & Issue Registry

Copyright (c) 2026 Edison Lepiten / AIEONYX
SPDX-License-Identifier: Apache-2.0

Last updated: C12 / v1.0.0 (commit pending)

---

## CLOSED BUGS

### KNOWN-BUG-001 — GTK Child Webview Height Constraint
- **Status:** RESOLVED (commit 8fcc84a)
- **Symptom:** Chrome and content webview split 50/50 instead of 108px/692px
- **Root cause:** Tauri v2 add_child() GtkFixed ignores size requests
- **Fix:** window.default_vbox() + pack_start(expand=false/true) repack surgery
- **CS Term:** TERM-045

---

## OPEN BUGS

### KNOWN-BUG-002 — Vite Dev Mode Black Gap
- **Status:** Open — dev mode only
- **Symptom:** ~52px black gap between chrome and content in cargo tauri dev
- **Root cause:** Vite hot-reload triggers GTK layout reset after startup surgery
- **Priority:** Low — does not affect production binary
- **Fix:** C12-SERVO track

### KNOWN-BUG-003 — WebKitWebView GTK Focus Expansion
- **Status:** Open — binary and dev mode
- **Symptom:** Chrome webview expands to fill window on first click
- **Root cause:** WebKitGTK natural-size-request overrides GtkBox pack constraints on focus
- **Attempts:** set_size_request, set_vexpand, ScrolledWindow, GTK CSS — all failed
- **Priority:** High — deferred C12-SERVO
- **CS Term:** TERM-046 related

### KNOWN-BUG-004 — Tab History Not Per-Tab
- **Status:** Open
- **Symptom:** Back/forward uses shared content webview history
- **Root cause:** Single shared content webview
- **Priority:** Medium — deferred C8+/Servo

### KNOWN-BUG-005 — Tab Switch Delay 1-3 Seconds
- **Status:** Open
- **Symptom:** Tab switch causes full page reload
- **Root cause:** Single content webview, must re-navigate on switch
- **Priority:** Medium — deferred EdisonDB cache

### KNOWN-BUG-006 — YouTube/Vimeo Skeleton/JS Error
- **Status:** Open — engine limitation
- **Symptom:** YouTube skeleton, Vimeo JS error
- **Root cause:** WebKitGTK engine compat limitations
- **Priority:** Low — C12-SERVO

### KNOWN-BUG-007 — YouTube Video Playback Blocked
- **Status:** Open — engine limitation
- **Symptom:** YouTube player shows error
- **Root cause:** Widevine DRM not available in WebKitGTK
- **Priority:** Low — C12-SERVO

---

## OPEN TECHNICAL DEBT

### AUDIT-001 — EdisonDB Critical Tier Unencrypted
- **Status:** Open — HIGH RISK
- **Finding:** Critical tier stores data in plaintext
- **Risk:** C9 vault credentials unprotected on disk
- **Mitigation:** C9 is UI prototype only, no real credentials stored
- **Fix:** EdisonDB Phase 4 encryption + client-side AES-256-GCM

### TECH-DEBT-001 — EdisonDB No Upsert
- **Status:** Open — workaround in place
- **Workaround:** Delete-then-write in session.rs
- **Risk:** Race condition on concurrent saves
- **Fix:** EdisonDB upsert endpoint

### TECH-DEBT-002 — EdisonDB Must Start Manually
- **Status:** Open
- **Risk:** Session not saved/restored if EdisonDB not running
- **Current:** Graceful degradation with warning
- **Fix:** Auto-start EdisonDB as sidecar (C10 target)

### TECH-DEBT-003 — Tab Titles Show URL Not Page Title
- **Status:** RESOLVED (this commit)
- **Fix:** connect_title_notify wired, title-changed event emitted

### TECH-DEBT-004 — SSV/STS Blocklists Hardcoded
- **Status:** Open
- **Risk:** Stale lists over time
- **Fix:** Load from EdisonDB, update via AWP

### TECH-DEBT-005 — Vault Credentials In-Memory Only
- **Status:** Open
- **Risk:** Credentials lost on browser close
- **Fix:** Wire to EdisonDB Critical tier when encryption lands

---

## KNOWN LIMITATIONS

| ID | Description | Resolution |
|----|-------------|------------|
| KNOWN-LIMITATION-001 | No Widevine DRM | C12-SERVO |
| KNOWN-LIMITATION-002 | WebKitGTK JS compat — Vimeo/heavy SPAs | C12-SERVO |
| KNOWN-LIMITATION-003 | Tab state lost on reorder | Future |
| KNOWN-LIMITATION-004 | AWP mesh routing placeholder only | C11+ |

---

## TRACK-C-SERVO — Deferred to v1.1

Servo engine evaluation via tauri-runtime-verso deferred from v1.0.
Reasons:
1. tauri-runtime-verso pins older Tauri versions — risks breaking C1-C11 features
2. multi-webview architecture (window.add_child) is WRY-specific, not yet supported in Verso
3. AIEONYX sovereign renderer (AXON Display Protocol) is the correct long-term path

v1.0 ships with WebKitGTK — open source, auditable, sovereign enough for initial release.
KNOWN-BUG-002 and KNOWN-BUG-003 deferred to v1.1 AXON Display Protocol track.
