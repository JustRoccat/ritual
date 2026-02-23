use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use anyhow::Result;
use serde_json::json;

use crate::{config::AiConfig, detector::Context, mapper::Status};

pub struct AiFallback {
    cfg: AiConfig,
    http: reqwest::Client,
    cache: HashMap<String, (Instant, Status)>,
}

impl AiFallback {
    pub fn new(cfg: AiConfig) -> Self {
        Self {
            cfg,
            http: reqwest::Client::new(),
            cache: HashMap::new(),
        }
    }

    pub async fn resolve(&mut self, ctx: &Context, recent_history: &[String]) -> Result<Status> {
        if !self.cfg.enabled {
            return Ok(Status::simple(format!("Using {}", ctx.app_name)));
        }

        let key = ctx.identity();
        if let Some((ts, status)) = self.cache.get(&key) {
            if ts.elapsed() < Duration::from_secs(self.cfg.cache_ttl_sec) {
                return Ok(status.clone());
            }
        }

        let prompt = json!({
            "app_name": ctx.app_name,
            "window_title": ctx.window_title,
            "recent_history": recent_history,
        });

        let payload = json!({
            "model": self.cfg.model,
            "messages": [
                {"role": "system", "content": "Generate a concise Discord Rich Presence status."},
                {"role": "user", "content": prompt.to_string()}
            ]
        });

        let mut request = self.http.post(&self.cfg.endpoint).json(&payload);
        request = match self.cfg.provider.as_str() {
            "openai" | "anthropic" => request.bearer_auth(&self.cfg.api_key),
            "local" => request,
            _ => request.bearer_auth(&self.cfg.api_key),
        };

        let response = request.send().await?;

        let value: serde_json::Value = response.json().await?;
        let generated = value["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("Exploring unknown realms")
            .to_string();

        let status = Status {
            details: generated,
            state: "Unknown app".to_string(),
            small_image: Some("ai".to_string()),
        };

        self.cache.insert(key, (Instant::now(), status.clone()));
        Ok(status)
    }
}
