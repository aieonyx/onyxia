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
