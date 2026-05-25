use std::fs;

use anyhow::Result;
use gpui::Global;

use crate::proxy::node::ProxyNode;
use crate::proxy::subscription::Subscription;
use crate::state::settings::AppSettings;

pub struct AppState {
    pub settings: AppSettings,
    pub nodes: Vec<ProxyNode>,
    pub subscriptions: Vec<Subscription>,
    pub selected_node_index: Option<usize>,
    pub selected_sub_index: Option<usize>,
}

impl Global for AppState {}

impl AppState {
    pub fn load() -> Self {
        let settings = AppSettings::load();
        let nodes = Self::load_nodes();
        let subscriptions = Self::load_subscriptions();

        Self {
            settings,
            nodes,
            subscriptions,
            selected_node_index: None,
            selected_sub_index: None,
        }
    }

    pub fn save_all(&self) {
        if let Err(e) = self.settings.save() {
            log::error!("Failed to save settings: {}", e);
        }
        if let Err(e) = self.save_nodes() {
            log::error!("Failed to save nodes: {}", e);
        }
        if let Err(e) = self.save_subscriptions() {
            log::error!("Failed to save subscriptions: {}", e);
        }
    }

    pub fn all_nodes(&self) -> Vec<&ProxyNode> {
        let mut all: Vec<&ProxyNode> = self.nodes.iter().collect();
        for sub in &self.subscriptions {
            for node in &sub.nodes {
                all.push(node);
            }
        }
        all
    }

    pub fn add_node(&mut self, node: ProxyNode) {
        self.nodes.push(node);
    }

    pub fn remove_node(&mut self, index: usize) {
        if index < self.nodes.len() {
            self.nodes.remove(index);
        }
    }

    pub fn add_subscription(&mut self, sub: Subscription) {
        self.subscriptions.push(sub);
    }

    fn load_nodes() -> Vec<ProxyNode> {
        let path = AppSettings::data_dir().join("nodes.json");
        if path.exists() {
            match fs::read_to_string(&path) {
                Ok(content) => match serde_json::from_str(&content) {
                    Ok(nodes) => return nodes,
                    Err(e) => log::warn!("Failed to parse nodes: {}", e),
                },
                Err(e) => log::warn!("Failed to read nodes file: {}", e),
            }
        }
        Vec::new()
    }

    fn save_nodes(&self) -> Result<()> {
        let path = AppSettings::data_dir().join("nodes.json");
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(&self.nodes)?;
        fs::write(path, json)?;
        Ok(())
    }

    fn load_subscriptions() -> Vec<Subscription> {
        let path = AppSettings::data_dir().join("subscriptions.json");
        if path.exists() {
            match fs::read_to_string(&path) {
                Ok(content) => match serde_json::from_str(&content) {
                    Ok(subs) => return subs,
                    Err(e) => log::warn!("Failed to parse subscriptions: {}", e),
                },
                Err(e) => log::warn!("Failed to read subscriptions file: {}", e),
            }
        }
        Vec::new()
    }

    fn save_subscriptions(&self) -> Result<()> {
        let path = AppSettings::data_dir().join("subscriptions.json");
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(&self.subscriptions)?;
        fs::write(path, json)?;
        Ok(())
    }
}
