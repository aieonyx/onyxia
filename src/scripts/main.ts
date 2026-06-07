
// Copyright (c) 2026 Edison Lepiten / AIEONYX
// C3: AXON-Client header injection via fetch interceptor.
// INVARIANT: header grants no elevated server permissions (SI-5).
// Covers all fetch/XHR requests. Top-level navigation headers: C4.

const AXON_CLIENT_HEADER = 'AXON-Client';
const AXON_CLIENT_VALUE = 'onyxia/0.1.0 (sovereign; linux)';

const originalFetch = window.fetch;
window.fetch = function(input: RequestInfo | URL, init?: RequestInit): Promise<Response> {
  const url = typeof input === 'string' ? input : input instanceof URL ? input.href : input.url;
  if (url.startsWith('http://') || url.startsWith('https://')) {
    init = init || {};
    init.headers = new Headers(init.headers || {});
    (init.headers as Headers).set(AXON_CLIENT_HEADER, AXON_CLIENT_VALUE);
  }
  return originalFetch.call(this, input, init);
};

// XHR intercept
const OriginalXHR = window.XMLHttpRequest;
class SovereignXHR extends OriginalXHR {
  open(method: string, url: string | URL, ...args: any[]): void {
    super.open(method, url, ...args);
    const urlStr = url.toString();
    if (urlStr.startsWith('http://') || urlStr.startsWith('https://')) {
      this.setRequestHeader(AXON_CLIENT_HEADER, AXON_CLIENT_VALUE);
    }
  }
}
window.XMLHttpRequest = SovereignXHR as any;

// Copyright (c) 2026 Edison Lepiten / AIEONYX
// C1 browser chrome — navigation and tab management only.
// INVARIANT: this script never touches vault data, AWIT tokens, or credentials.

import { invoke } from '@tauri-apps/api/core';

interface Tab {
  id: number;
  url: string;
  title: string;
  trust: string;
}

let tabs: Tab[] = [{ id: 1, url: 'about:blank', title: 'New Tab', trust: 'unknown' }];
let activeTabId = 1;

// DOM references
const urlInput = document.getElementById('url-input') as HTMLInputElement;
const tabsContainer = document.getElementById('tabs-container') as HTMLDivElement;
const btnBack = document.getElementById('btn-back') as HTMLButtonElement;
const btnForward = document.getElementById('btn-forward') as HTMLButtonElement;
const btnReload = document.getElementById('btn-reload') as HTMLButtonElement;
const newTabBtn = document.getElementById('new-tab-btn') as HTMLButtonElement;
const trustIndicator = document.getElementById('trust-indicator') as HTMLSpanElement;

function renderTabs(): void {
  tabsContainer.innerHTML = '';
  tabs.forEach(tab => {
    const el = document.createElement('div');
    el.className = `tab${tab.id === activeTabId ? ' active' : ''}`;
    el.dataset.tabId = String(tab.id);

    const titleSpan = document.createElement('span');
    titleSpan.className = 'tab-title';
    titleSpan.textContent = tab.title || 'New Tab';

    const closeBtn = document.createElement('button');
    closeBtn.className = 'tab-close';
    closeBtn.textContent = '×';
    closeBtn.title = 'Close tab';
    closeBtn.addEventListener('click', (e) => {
      e.stopPropagation();
      closeTab(tab.id);
    });

    el.appendChild(titleSpan);
    el.appendChild(closeBtn);
    el.addEventListener('click', () => switchTab(tab.id));
    tabsContainer.appendChild(el);
  });

  // Update URL bar to show active tab URL
  const activeTab = tabs.find(t => t.id === activeTabId);
  if (activeTab) {
    urlInput.value = activeTab.url === 'about:blank' ? '' : activeTab.url;
    updateTrustIndicator(activeTab.trust);
  }
}

// Trust indicator — C1: always empty/unknown.
// C6 will populate this from TrustIndicator enum sent by main process.
function updateTrustIndicator(trust: string): void {
  trustIndicator.textContent = '';
  trustIndicator.title = '';
  // C6 implementation:
  // 'sovereign_star' → '✶'
  // 'legacy_https'   → '🔒'
  // 'legacy_http'    → '⚠️'
  // 'aieonyx_ca'     → '🔵'
}

async function navigate(url: string): Promise<void> {
  if (!url.trim()) return;
  try {
    await invoke('navigate', { url });
    const activeTab = tabs.find(t => t.id === activeTabId);
    if (activeTab) {
      const normalized = url.startsWith('http') ? url : `https://${url}`;
      activeTab.url = normalized;
      activeTab.title = normalized;
    }
    renderTabs();
  } catch (err) {
    console.error('Navigation error:', err);
  }
}

async function newTab(): Promise<void> {
  try {
    const tab = await invoke<Tab>('new_tab');
    tabs.push(tab);
    activeTabId = tab.id;
    renderTabs();
  } catch (err) {
    console.error('New tab error:', err);
  }
}

async function closeTab(id: number): Promise<void> {
  try {
    await invoke('close_tab', { id });
    tabs = tabs.filter(t => t.id !== id);
    if (activeTabId === id && tabs.length > 0) {
      activeTabId = tabs[tabs.length - 1].id;
    }
    renderTabs();
  } catch (err) {
    console.error('Close tab error:', err);
  }
}

async function switchTab(id: number): Promise<void> {
  try {
    await invoke('switch_tab', { id });
    activeTabId = id;
    renderTabs();
  } catch (err) {
    console.error('Switch tab error:', err);
  }
}

// Event listeners
urlInput.addEventListener('keydown', (e) => {
  if (e.key === 'Enter') navigate(urlInput.value);
});

btnBack.addEventListener('click', () => invoke('go_back'));
btnForward.addEventListener('click', () => invoke('go_forward'));
btnReload.addEventListener('click', () => {
  invoke('navigate', { url: urlInput.value });
});
newTabBtn.addEventListener('click', newTab);

// Initial render
renderTabs();
