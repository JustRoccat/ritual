use std::{fs, path::Path};

use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub presence: PresenceConfig,
    #[serde(default)]
    pub ai: AiConfig,
    #[serde(default = "default_discord_socket")]
    pub discord_socket: String,
    #[serde(default = "default_discord_client_id")]
    pub discord_client_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PresenceConfig {
    #[serde(default = "default_debounce")]
    pub debounce_sec: u64,
}

impl Default for PresenceConfig {
    fn default() -> Self {
        Self { debounce_sec: 3 }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AiConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub provider: String,
    #[serde(default)]
    pub api_key: String,
    #[serde(default = "default_model")]
    pub model: String,
    #[serde(default = "default_endpoint")]
    pub endpoint: String,
    #[serde(default = "default_cache_ttl")]
    pub cache_ttl_sec: u64,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            provider: String::new(),
            api_key: String::new(),
            model: default_model(),
            endpoint: default_endpoint(),
            cache_ttl_sec: default_cache_ttl(),
        }
    }
}

fn default_debounce() -> u64 {
    3
}
fn default_model() -> String {
    "gpt-5-mini".to_string()
}
fn default_endpoint() -> String {
    "https://api.openai.com/v1/chat/completions".to_string()
}
fn default_cache_ttl() -> u64 {
    3600
}
fn default_discord_socket() -> String {
    "/run/user/{uid}/discord-ipc-0".to_string()
}
fn default_discord_client_id() -> String {
    "YOUR_DISCORD_APP_ID".to_string()
}

impl Config {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let raw = fs::read_to_string(path)?;
        Ok(toml::from_str(&raw)?)
    }

    pub fn discord_socket_for_uid(&self, uid: u32) -> String {
        self.discord_socket.replace("{uid}", &uid.to_string())
    }
}
