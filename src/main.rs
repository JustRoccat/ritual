mod ai;
mod config;
mod detector;
mod discord;
mod mapper;
mod plugins;

use std::{collections::VecDeque, time::Duration};

use anyhow::Result;
use detector::x11::X11Detector;
use mapper::{StaticMapper, Status};
use plugins::{generic::GenericPlugin, steam::SteamPlugin, vscode::VsCodePlugin, Plugin};

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = config::Config::load("config.toml")?;
    let uid = std::env::var("UID")
        .ok()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(1000);

    let detector = X11Detector::new()?;
    let mapper = StaticMapper::new();
    let mut ai = ai::AiFallback::new(cfg.ai.clone());
    let discord = discord::DiscordRpc::new(
        cfg.discord_socket_for_uid(uid),
        cfg.discord_client_id.clone(),
        std::process::id(),
    );

    let plugins: Vec<Box<dyn Plugin>> = vec![
        Box::new(VsCodePlugin),
        Box::new(SteamPlugin),
        Box::new(GenericPlugin),
    ];

    let mut last_status: Option<Status> = None;
    let mut history: VecDeque<String> = VecDeque::with_capacity(20);

    loop {
        let ctx = match detector.active_context() {
            Ok(ctx) => ctx,
            Err(_) => {
                tokio::time::sleep(Duration::from_secs(1)).await;
                continue;
            }
        };

        let status = if let Some(plugin) = plugins.iter().find(|p| p.supports(&ctx)) {
            plugin.generate(&ctx)
        } else if let Some(mapped) = mapper.map(&ctx.app_name) {
            mapped
        } else {
            let hist: Vec<String> = history.iter().cloned().collect();
            ai.resolve(&ctx, &hist).await?
        };

        if last_status.as_ref() != Some(&status) {
            if let Err(err) = discord.publish(&status).await {
                eprintln!("discord publish failed: {err}");
            }
            last_status = Some(status.clone());
            if history.len() >= 20 {
                history.pop_front();
            }
            history.push_back(ctx.identity());
        }

        tokio::time::sleep(Duration::from_secs(cfg.presence.debounce_sec)).await;
    }
}
