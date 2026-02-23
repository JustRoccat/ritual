use crate::detector::Context;
use crate::mapper::Status;

use super::Plugin;

pub struct SteamPlugin;

impl Plugin for SteamPlugin {
    fn supports(&self, ctx: &Context) -> bool {
        ctx.app_name == "steam"
    }

    fn generate(&self, ctx: &Context) -> Status {
        Status {
            details: "Avoiding responsibilities".to_string(),
            state: format!("{}", ctx.window_title),
            small_image: Some("steam".to_string()),
        }
    }
}
