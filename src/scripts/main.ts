// Copyright (c) 2026 Edison Lepiten / AIEONYX
// C5-A: Protocol switcher + ARPi status bar
// INVARIANT: protocol and ARPi state always computed by Rust main process.
// INVARIANT: this script never touches vault data, AWIT tokens, or credentials.

import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { Window, getCurrent, getAll } from "@tauri-apps/api/window";

interface ArpiStatus {
  schema: string;
  identity: string;
  mutual_auth: string;
  scope: string;
  anomaly: string;
}

interface PageState {
  protocol: string;
  host: string;
  arpi: ArpiStatus;
}

interface Tab {
  id: number;
  url: string;
  title: string;
  trust: string;
}

let tabs: Tab[] = [{ id: 1, url: "about:blank", title: "New Tab", trust: "unknown" }];
let activeTabId = 1;
let isSwitchingTab = false;
let isLoadingSession = false;

// DOM references
const urlInput = document.getElementById("url-input") as HTMLInputElement;
const tabsContainer = document.getElementById("tabs-container") as HTMLDivElement;
const btnBack = document.getElementById("btn-back") as HTMLButtonElement;
const btnForward = document.getElementById("btn-forward") as HTMLButtonElement;
const btnReload = document.getElementById("btn-reload") as HTMLButtonElement;
const newTabBtn = document.getElementById("new-tab-btn") as HTMLButtonElement;
const protocolBtn = document.getElementById("protocol-btn") as HTMLButtonElement;
const protocolIcon = document.getElementById("protocol-icon") as HTMLSpanElement;
const protocolDropdown = document.getElementById("protocol-dropdown") as HTMLDivElement;
const httpsCheck = document.getElementById("https-check") as HTMLSpanElement;
const awpCheck = document.getElementById("awp-check") as HTMLSpanElement;
const arpiBar = document.getElementById("arpi-bar") as HTMLDivElement;
const trustIndicator = document.getElementById("trust-indicator") as HTMLSpanElement;
const arpiLegacy = document.getElementById("arpi-legacy") as HTMLDivElement;
const arpiSovereign = document.getElementById("arpi-sovereign") as HTMLDivElement;

const layerEls: Record<string, HTMLElement> = {
  schema: document.getElementById("layer-schema")!,
  identity: document.getElementById("layer-identity")!,
  mutual_auth: document.getElementById("layer-auth")!,
  scope: document.getElementById("layer-scope")!,
  anomaly: document.getElementById("layer-anomaly")!,
};

// ── Protocol switcher ────────────────────────────────────────
protocolBtn.addEventListener("click", (e) => {
  e.stopPropagation();
  protocolDropdown.classList.toggle("hidden");
});

document.addEventListener("click", () => {
  protocolDropdown.classList.add("hidden");
});

document.querySelectorAll(".protocol-option").forEach((btn) => {
  btn.addEventListener("click", async () => {
    const proto = (btn as HTMLElement).dataset.protocol;
    const currentUrl = urlInput.value;
    if (!currentUrl || !proto) return;
    let newUrl = currentUrl;
    if (proto === "awp") {
      newUrl = currentUrl.replace(/^https?:\/\//, "awp://");
    } else if (proto === "https") {
      newUrl = currentUrl.replace(/^awp:\/\//, "https://");
    }
    protocolDropdown.classList.add("hidden");
    await navigate(newUrl);
  });
});

// ── ARPi bar ─────────────────────────────────────────────────
async function updatePageState(url: string): Promise<void> {
  try {
    const state = await invoke<PageState>("update_page_url", { url });
    renderPageState(state);
  } catch (err) {
    console.error("Page state update error:", err);
  }
}

function renderPageState(state: PageState): void {
  if (state.protocol === "awp" && !state.host.includes("asset")) {
    protocolIcon.textContent = "✶";
    awpCheck.textContent = "✓";
    httpsCheck.textContent = "";
    trustIndicator.textContent = "⬡";
    trustIndicator.className = "trust-sovereign";
    trustIndicator.title = "Sovereign connection — AWP active";
  } else if (state.protocol === "https") {
    protocolIcon.textContent = "🔒";
    httpsCheck.textContent = "✓";
    awpCheck.textContent = "";
    trustIndicator.textContent = "●";
    trustIndicator.className = "trust-https";
    trustIndicator.title = "HTTPS — encrypted, legacy connection";
  } else if (state.protocol === "http") {
    protocolIcon.textContent = "⚠";
    httpsCheck.textContent = "";
    awpCheck.textContent = "";
    trustIndicator.textContent = "▲";
    trustIndicator.className = "trust-http";
    trustIndicator.title = "Insecure connection — no TLS";
  } else {
    protocolIcon.textContent = "";
    httpsCheck.textContent = "";
    awpCheck.textContent = "";
    trustIndicator.textContent = "○";
    trustIndicator.className = "trust-unknown";
    trustIndicator.title = "No connection";
  }

  if (state.protocol === "none" || !state.protocol) {
    arpiBar.classList.add("hidden");
    return;
  }

  arpiBar.classList.remove("hidden");

  if (state.protocol === "awp" && !state.host.includes("asset")) {
    arpiLegacy.classList.add("hidden");
    arpiSovereign.classList.remove("hidden");
    Object.entries(layerEls).forEach(([key, el]) => {
      el.className = "arpi-layer";
      const status = (state.arpi as any)[key] as string;
      if (status === "verified") el.classList.add("verified");
      else if (status === "pending") el.classList.add("pending");
      else if (status === "failed") el.classList.add("failed");
    });
  } else {
    arpiLegacy.classList.remove("hidden");
    arpiSovereign.classList.add("hidden");
  }
}

// ── Tab management ───────────────────────────────────────────
function renderTabs(): void {
  tabsContainer.innerHTML = "";
  tabs.forEach((tab) => {
    const el = document.createElement("div");
    el.className = `tab${tab.id === activeTabId ? " active" : ""}`;
    el.dataset.tabId = String(tab.id);
    const titleSpan = document.createElement("span");
    titleSpan.className = "tab-title";
    titleSpan.textContent = tab.title || "New Tab";
    const closeBtn = document.createElement("button");
    closeBtn.className = "tab-close";
    closeBtn.textContent = "×";
    closeBtn.title = "Close tab";
    closeBtn.addEventListener("click", (e) => {
      e.stopPropagation();
      closeTab(tab.id);
    });
    el.appendChild(titleSpan);
    el.appendChild(closeBtn);
    el.addEventListener("click", () => { if (!didDrag) switchTab(tab.id); });

    // C6-B: Tab drag-to-reorder
    let didDrag = false;
    el.draggable = true;
    el.addEventListener("dragstart", (e) => {
      didDrag = true;
      el.classList.add("dragging");
      e.dataTransfer?.setData("text/plain", String(tab.id));
    });
    el.addEventListener("dragend", () => {
      el.classList.remove("dragging");
      document.querySelectorAll(".tab").forEach(t => t.classList.remove("drag-over"));
      setTimeout(() => { didDrag = false; }, 50);
    });
    el.addEventListener("dragover", (e) => {
      e.preventDefault();
      document.querySelectorAll(".tab").forEach(t => t.classList.remove("drag-over"));
      el.classList.add("drag-over");
    });
    el.addEventListener("drop", (e) => {
      e.preventDefault();
      el.classList.remove("drag-over");
      const draggedId = Number(e.dataTransfer?.getData("text/plain"));
      const targetId = tab.id;
      if (draggedId === targetId) return;
      const fromIdx = tabs.findIndex(t => t.id === draggedId);
      const toIdx   = tabs.findIndex(t => t.id === targetId);
      if (fromIdx === -1 || toIdx === -1) return;
      const moved = tabs.splice(fromIdx, 1)[0];
      tabs.splice(toIdx, 0, moved);
      renderTabs();
    });

    tabsContainer.appendChild(el);
  });
  const activeTab = tabs.find((t) => t.id === activeTabId);
  if (activeTab) {
    urlInput.value = activeTab.url === "about:blank" ? "" : activeTab.url;
  }
}

// ── Navigation ───────────────────────────────────────────────
async function navigate(url: string): Promise<void> {
  if (!url.trim()) return;
  try {
    await invoke("navigate", { url });
    const activeTab = tabs.find((t) => t.id === activeTabId);
    if (activeTab) {
      const normalized = url.startsWith("http") || url.startsWith("awp")
        ? url
        : `https://${url}`;
      activeTab.url = normalized;
      activeTab.title = normalized;
      urlInput.value = normalized;
      await updatePageState(normalized);
    }
    renderTabs();
    saveSessionDebounced();
  } catch (err) {
    console.error("Navigation error:", err);
  }
}

async function newTab(): Promise<void> {
  try {
    const tab = await invoke<Tab>("new_tab");
    tabs.push(tab);
    activeTabId = tab.id;
    renderTabs();
    await updatePageState("about:blank");
    saveSessionDebounced();
  } catch (err) {
    console.error("New tab error:", err);
  }
}

async function closeTab(id: number): Promise<void> {
  try {
    await invoke("close_tab", { id });
    tabs = tabs.filter((t) => t.id !== id);
    if (activeTabId === id && tabs.length > 0) {
      activeTabId = tabs[tabs.length - 1].id;
    }
    renderTabs();
    saveSessionDebounced();
  } catch (err) {
    console.error("Close tab error:", err);
  }
}

async function switchTab(id: number): Promise<void> {
  try {
    isSwitchingTab = true;
    await invoke("switch_tab", { id });
    activeTabId = id;
    renderTabs();
    setTimeout(() => { isSwitchingTab = false; }, 1500);
    saveSessionDebounced();
  } catch (err) {
    isSwitchingTab = false;
    console.error("Switch tab error:", err);
  }
}

// ── Event listeners ──────────────────────────────────────────
urlInput.addEventListener("keydown", (e) => {
  if (e.key === "Enter") navigate(urlInput.value);
});

btnBack.addEventListener("click", () => invoke("go_back"));
btnForward.addEventListener("click", () => invoke("go_forward"));
btnReload.addEventListener("click", () => navigate(urlInput.value));
newTabBtn.addEventListener("click", newTab);

// Initial render
renderTabs();
updatePageState("about:blank");

// C8: EdisonDB session persistence
interface SessionTab {
  id: number;
  url: string;
  title: string;
}

async function saveSession(): Promise<void> {
  console.log("SAVE SESSION: tabs count =", tabs.length, tabs.map(t => t.url));
  try {
    const sessionTabs: SessionTab[] = tabs.map(t => ({
      id: t.id,
      url: t.url === "about:blank" ? "" : t.url,
      title: t.title
    }));
    await invoke("save_session", { tabs: sessionTabs, activeId: activeTabId });
  } catch (err) {
    console.warn("Session save FAILED:", err);
    // Write error to URL bar briefly
    urlInput.value = "DB ERR: " + String(err).slice(0,40);
    setTimeout(() => { urlInput.value = ""; }, 3000);
  }
}

async function loadSession(): Promise<void> {
  isLoadingSession = true;
  try {
    const session = await invoke<{ tabs: SessionTab[]; active: number } | null>("load_session");
    if (session && session.tabs.length > 0) {
      tabs = session.tabs.map(t => ({
        id: t.id,
        url: t.url || "about:blank",
        title: t.title || "New Tab",
        trust: "unknown"
      }));
      activeTabId = session.active;
      renderTabs();
      // Navigate to active tab URL
      // Restore tab UI only — user clicks to navigate (avoids WebKit loop)
      isLoadingSession = false;
      renderTabs();
      const activeTab = tabs.find(t => t.id === activeTabId);
      if (activeTab && activeTab.url && activeTab.url !== "about:blank") {
        urlInput.value = activeTab.url;
      }
      console.log("Session restored:", tabs.length, "tabs");
    }
  } catch (err) {
    console.warn("Session load failed (EdisonDB offline?):", err);
  }
}

// Auto-save session on tab changes
async function saveSessionDebounced(): Promise<void> {
  if (isLoadingSession) return;
  await saveSession([...tabs]);
}

// C8: Load session on startup
loadSession();

// Window controls via Rust commands
document.getElementById("btn-minimize")?.addEventListener("click", () => invoke("minimize_window"));
document.getElementById("btn-maximize")?.addEventListener("click", () => invoke("maximize_window"));
document.getElementById("btn-close")?.addEventListener("click", () => invoke("close_window"));

// Window drag — child webview must invoke start_dragging explicitly
document.getElementById("tab-strip")?.addEventListener("mousedown", (e) => {
    const target = e.target as HTMLElement;
    // Only drag on empty strip area, not on tabs or buttons
    if (target.id === "tab-strip" || target.id === "tabs-container") {
        invoke("start_drag");
    }
});

// Sync URL bar when content webview navigates (e.g. search, redirects)
listen<string>("url-changed", (event) => {
    if (isSwitchingTab) return;
    const url = event.payload;
    // Filter out tracker/sub-resource URLs that are not top-level navigations
    const trackerPatterns = [
      "googletagmanager.com", "google-analytics.com", "doubleclick.net",
      "googlesyndication.com", "facebook.net", "hotjar.com", "clarity.ms"
    ];
    if (trackerPatterns.some(t => url.includes(t))) return;
    const activeTab = tabs.find(t => t.id === activeTabId);
    if (activeTab) {
        activeTab.url = url;
        activeTab.title = url;
        urlInput.value = url;
        updatePageState(url);
        renderTabs();
        // Sync URL back to Rust TabManager so switch_tab restores correctly
        invoke("update_page_url", { url }).catch(() => {});
        saveSessionDebounced();
    }
});

// C7-A: STS threat detection listener
interface ThreatEvent {
  kind: string;
  domain: string;
  url: string;
}

let threatCount = 0;

listen<ThreatEvent>("threat-detected", (event) => {
  const threat = event.payload;
  threatCount++;
  console.warn("STS threat:", threat.kind, threat.domain);

  // Flash anomaly layer red
  const anomalyEl = layerEls["anomaly"];
  if (anomalyEl) {
    anomalyEl.className = "arpi-layer failed";
    anomalyEl.title = threat.kind + ": " + threat.domain;
  }

  // Show ARPi bar if hidden
  arpiBar.classList.remove("hidden");
  arpiLegacy.classList.add("hidden");
  arpiSovereign.classList.remove("hidden");

  // Update trust indicator to warning
  trustIndicator.textContent = "▲";
  trustIndicator.className = "trust-http";
  trustIndicator.title = "STS: " + threatCount + " threat(s) detected";
});

// C7-B: SSV blocked navigation listener
interface SsvVerdict {
  threat: ThreatEvent;
  suggested_real: string | null;
  block: boolean;
}

listen<SsvVerdict>("ssv-blocked", (event) => {
  const v = event.payload;
  const kind = v.threat.kind.replace(/_/g, " ");
  const suggested = v.suggested_real ? "Did you mean: " + v.suggested_real : "";
  const msg = "SOVEREIGN BLOCK: " + kind + " — " + v.threat.domain + (suggested ? ". " + suggested : "");

  // Show in URL bar as warning
  urlInput.value = "BLOCKED: " + v.threat.domain;
  urlInput.style.color = "#FF3D00";
  setTimeout(() => {
    urlInput.style.color = "";
    urlInput.value = "";
  }, 5000);

  // Flash anomaly layer
  const anomalyEl = layerEls["anomaly"];
  if (anomalyEl) {
    anomalyEl.className = "arpi-layer failed";
    anomalyEl.title = msg;
  }
  arpiBar.classList.remove("hidden");
  arpiLegacy.classList.add("hidden");
  arpiSovereign.classList.remove("hidden");
  trustIndicator.textContent = "✗";
  trustIndicator.className = "trust-http";
  trustIndicator.title = msg;

  console.warn("SSV blocked:", v);
});

// ── C9: Vault UI ─────────────────────────────────────────────
const vaultBtn = document.getElementById("vault-btn") as HTMLButtonElement;
const vaultPanel = document.getElementById("vault-panel") as HTMLDivElement;
const vaultClose = document.getElementById("vault-close") as HTMLButtonElement;
const vaultUnlockBtn = document.getElementById("vault-unlock-btn") as HTMLButtonElement;
const vaultLockStatus = document.getElementById("vault-lock-status") as HTMLSpanElement;
const vaultCredentials = document.getElementById("vault-credentials") as HTMLDivElement;
const vaultList = document.getElementById("vault-list") as HTMLDivElement;
const saveBanner = document.getElementById("save-credential-banner") as HTMLDivElement;
const saveCredHost = document.getElementById("save-cred-host") as HTMLElement;
const saveCredYes = document.getElementById("save-cred-yes") as HTMLButtonElement;
const saveCredNo = document.getElementById("save-cred-no") as HTMLButtonElement;
const saveCredDismiss = document.getElementById("save-cred-dismiss") as HTMLButtonElement;

let vaultUnlocked = false;
let vaultCredentialsList: { domain: string; username: string; note: string }[] = [];

// Toggle vault panel
vaultBtn.addEventListener("click", (e) => {
  e.stopPropagation();
  vaultPanel.classList.toggle("hidden");
});

vaultClose.addEventListener("click", () => {
  vaultPanel.classList.add("hidden");
});

document.addEventListener("click", (e) => {
  const t = e.target as Node;
  if (!vaultPanel.contains(t) && e.target !== vaultBtn) {
    vaultPanel.classList.add("hidden");
  }
// Vault unlock (C9-UI: master password prompt)
});

vaultUnlockBtn.addEventListener("click", () => {
  const masterPwd = prompt("Enter master password:");
  // C9-UI: accept any non-empty password for UI prototype
  // Real encryption added when EdisonDB P4 encryption lands
  vaultUnlocked = true;
  vaultLockStatus.textContent = "Vault unlocked";
  vaultLockStatus.style.color = "#00C853";
  vaultUnlockBtn.textContent = "Lock";
  vaultBtn.classList.add("unlocked");
  vaultCredentials.classList.remove("hidden");
  renderVaultList();
  vaultUnlockBtn.onclick = () => {
    vaultUnlocked = false;
    vaultLockStatus.textContent = "Vault locked";
    vaultLockStatus.style.color = "";
    vaultUnlockBtn.textContent = "Unlock";
    vaultBtn.classList.remove("unlocked");
    vaultCredentials.classList.add("hidden");
    vaultUnlockBtn.onclick = null;
  };
});

function renderVaultList(): void {
  vaultList.innerHTML = "";
  if (vaultCredentialsList.length === 0) {
    vaultList.innerHTML = "<div style='color:var(--text-secondary);font-size:12px;padding:8px'>No saved credentials</div>";
    return;
  }
  vaultCredentialsList.forEach(cred => {
    const el = document.createElement("div");
    el.className = "vault-entry";
    el.innerHTML = `
      <div>
        <div class="vault-entry-domain">${cred.domain}</div>
        <div class="vault-entry-user">${cred.username}</div>
      </div>
      <button class="vault-fill-btn" data-domain="${cred.domain}">Fill</button>
    `;
    vaultList.appendChild(el);
  });
}

// Save credential banner handlers
saveCredYes.addEventListener("click", () => {
  const host = saveCredHost.textContent || "";
  vaultCredentialsList.push({ domain: host, username: "(saved)", note: "C9-UI prototype" });
  saveBanner.classList.add("hidden");
  renderVaultList();
});

saveCredNo.addEventListener("click", () => saveBanner.classList.add("hidden"));
saveCredDismiss.addEventListener("click", () => saveBanner.classList.add("hidden"));

// Show save banner when navigating to a new HTTPS site (C9-UI demo)
listen<string>("url-changed", (event) => {
  const url = event.payload;
  if (url.startsWith("https://") && url.indexOf("google") === -1 && url.indexOf("about") === -1) {
    const host = url.replace("https://", "").split("/")[0];
    const alreadySaved = vaultCredentialsList.some(c => c.domain === host);
    if (alreadySaved === false) {
      saveCredHost.textContent = host;
      saveBanner.classList.remove("hidden");
    }
  }
});

// TECH-DEBT-003 fix: update tab title from page title
listen<string>("title-changed", (event) => {
  const title = event.payload;
  const activeTab = tabs.find(t => t.id === activeTabId);
  if (activeTab && title) {
    activeTab.title = title;
    renderTabs();
    saveSessionDebounced();
  }
});

// ── C10: Digital Legacy ──────────────────────────────────────
const legacyBtn = document.getElementById("legacy-btn") as HTMLButtonElement;
const legacyPanel = document.getElementById("legacy-panel") as HTMLDivElement;
const legacyClose = document.getElementById("legacy-close") as HTMLButtonElement;
const legacyDays = document.getElementById("legacy-days") as HTMLInputElement;
const legacyHolderName = document.getElementById("legacy-holder-name") as HTMLInputElement;
const legacyHolderContact = document.getElementById("legacy-holder-contact") as HTMLInputElement;
const legacyCriticalAction = document.getElementById("legacy-critical-action") as HTMLSelectElement;
const legacyPersonalAction = document.getElementById("legacy-personal-action") as HTMLSelectElement;
const legacyNoiseAction = document.getElementById("legacy-noise-action") as HTMLSelectElement;
const legacySaveBtn = document.getElementById("legacy-save-btn") as HTMLButtonElement;
const legacyStatusMsg = document.getElementById("legacy-status-msg") as HTMLDivElement;

// C10: Navigate to awp://legacy sovereign page
legacyBtn.addEventListener("click", (e) => {
  e.stopPropagation();
  navigate("awp://legacy");
});

legacyClose.addEventListener("click", () => legacyPanel.classList.add("hidden"));

document.addEventListener("click", (e) => {
  const target = e.target as Node;
  if (!legacyPanel.contains(target) && e.target !== legacyBtn) {
    legacyPanel.classList.add("hidden");
  }
});

// Load existing testament
async function loadLegacyData(): Promise<void> {
  try {
    const testament = await invoke<{
      inactivity_days: number;
      legacy_holder_name: string;
      legacy_holder_contact: string;
      critical_action: string;
      personal_action: string;
      noise_action: string;
    } | null>("load_testament");

    if (testament) {
      legacyDays.value = String(testament.inactivity_days);
      legacyHolderName.value = testament.legacy_holder_name;
      legacyHolderContact.value = testament.legacy_holder_contact;
      legacyCriticalAction.value = testament.critical_action;
      legacyPersonalAction.value = testament.personal_action;
      legacyNoiseAction.value = testament.noise_action;
      legacyStatusMsg.textContent = "Testament loaded from vault";
      legacyStatusMsg.style.color = "#4ade80";
    }
  } catch (err) {
    legacyStatusMsg.textContent = "EdisonDB offline";
    legacyStatusMsg.style.color = "#f87171";
  }
}

// Save testament
legacySaveBtn.addEventListener("click", async () => {
  const testament = {
    inactivity_days: parseInt(legacyDays.value) || 180,
    legacy_holder_name: legacyHolderName.value.trim(),
    legacy_holder_contact: legacyHolderContact.value.trim(),
    critical_action: legacyCriticalAction.value,
    personal_action: legacyPersonalAction.value,
    noise_action: legacyNoiseAction.value,
    created_at: Math.floor(Date.now() / 1000),
  };

  try {
    await invoke("save_testament", { testament });
    legacyStatusMsg.textContent = "Testament saved to sovereign vault";
    legacyStatusMsg.style.color = "#4ade80";
    legacyBtn.className = "legacy-active";
    updateLegacyStatus();
  } catch (err) {
    legacyStatusMsg.textContent = "Save failed: " + String(err).slice(0, 40);
    legacyStatusMsg.style.color = "#f87171";
  }
});

// Update legacy status indicator
async function updateLegacyStatus(): Promise<void> {
  try {
    const status = await invoke<{
      status: string;
      days_inactive: number;
      testament_active: boolean;
      holder: string;
    }>("get_legacy_status");

    legacyBtn.title = "Digital Legacy — " + status.status;

    if (status.status === "active") {
      legacyBtn.className = "legacy-active";
    } else if (status.status === "warning") {
      legacyBtn.className = "legacy-warning";
    } else if (status.status === "triggered") {
      legacyBtn.className = "legacy-triggered";
    } else {
      legacyBtn.className = "legacy-inactive";
    }
  } catch (_) {
    legacyBtn.className = "legacy-inactive";
  }
}

// Ping heartbeat on startup and update status
async function initLegacy(): Promise<void> {
  try {
    await invoke("ping_heartbeat");
    await updateLegacyStatus();
  } catch (_) {
    // EdisonDB offline — graceful degradation
  }
}

initLegacy();

// C11: Aegis Threat Intel
const aegisBtn = document.getElementById("aegis-btn") as HTMLButtonElement;
let sessionThreatCount = 0;

aegisBtn.addEventListener("click", (e) => {
  e.stopPropagation();
  navigate("awp://aegis");
});

// Track threats for Aegis button indicator
listen<{kind: string; domain: string; url: string}>("threat-detected", () => {
  sessionThreatCount++;
  aegisBtn.className = "threat-active";
  aegisBtn.title = "Aegis Threat Intel — " + sessionThreatCount + " threat(s)";
});
