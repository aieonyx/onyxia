// Copyright (c) 2026 Edison Lepiten / AIEONYX
// C5-A: Protocol switcher + ARPi status bar
// INVARIANT: protocol and ARPi state always computed by Rust main process.
// INVARIANT: this script never touches vault data, AWIT tokens, or credentials.

import { invoke } from "@tauri-apps/api/core";

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
  } else if (state.protocol === "https") {
    protocolIcon.textContent = "🔒";
    httpsCheck.textContent = "✓";
    awpCheck.textContent = "";
  } else {
    protocolIcon.textContent = "";
    httpsCheck.textContent = "";
    awpCheck.textContent = "";
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
    el.addEventListener("click", () => switchTab(tab.id));
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
    await invoke("switch_tab", { id });
    activeTabId = id;
    renderTabs();
  } catch (err) {
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
