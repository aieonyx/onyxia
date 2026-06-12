// Copyright (c) 2026 Edison Lepiten / AIEONYX
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use crate::browser::trust_state::TrustIndicator;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tab {
    pub id: u32,
    pub url: String,
    pub title: String,
    pub trust: TrustIndicator,
}

#[derive(Debug)]
pub struct TabManager {
    tabs: Vec<Tab>,
    active_id: u32,
    next_id: u32,
}

impl TabManager {
    pub fn new() -> Self {
        let first_tab = Tab {
            id: 1,
            url: String::from("about:blank"),
            title: String::from("New Tab"),
            trust: TrustIndicator::Unknown,
        };
        TabManager {
            tabs: vec![first_tab],
            active_id: 1,
            next_id: 2,
        }
    }

    pub fn new_tab(&mut self) -> &Tab {
        let id = self.next_id;
        self.next_id += 1;
        self.tabs.push(Tab {
            id,
            url: String::from("about:blank"),
            title: String::from("New Tab"),
            trust: TrustIndicator::Unknown,
        });
        self.active_id = id;
        self.tabs.last().unwrap()
    }

    pub fn close_tab(&mut self, id: u32) -> Result<(), String> {
        if self.tabs.len() == 1 {
            return Err("Cannot close last tab".to_string());
        }
        self.tabs.retain(|t| t.id != id);
        if self.active_id == id {
            self.active_id = self.tabs.last().unwrap().id;
        }
        Ok(())
    }

    pub fn switch_tab(&mut self, id: u32) -> Result<(), String> {
        if self.tabs.iter().any(|t| t.id == id) {
            self.active_id = id;
            Ok(())
        } else {
            Err(format!("Tab {} not found", id))
        }
    }

    pub fn active_id(&self) -> u32 {
        self.active_id
    }

    pub fn active_tab(&self) -> Option<&Tab> {
        self.tabs.iter().find(|t| t.id == self.active_id)
    }

    #[allow(dead_code)]
    pub fn tabs(&self) -> &Vec<Tab> {
        &self.tabs
    }

    pub fn update_tab_url(&mut self, id: u32, url: String, title: String) {
        if let Some(tab) = self.tabs.iter_mut().find(|t| t.id == id) {
            tab.url = url;
            tab.title = title;
            // Trust computed in C3/C6 — placeholder for now
            tab.trust = TrustIndicator::Unknown;
        }
    }
}

impl Default for TabManager {
    fn default() -> Self {
        Self::new()
    }
}
