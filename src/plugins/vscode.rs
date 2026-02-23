use crate::detector::Context;
use crate::mapper::Status;

use super::Plugin;

pub struct VsCodePlugin;

impl Plugin for VsCodePlugin {
    fn supports(&self, ctx: &Context) -> bool {
        matches!(ctx.app_name.as_str(), "code" | "vscode")
    }

    fn generate(&self, ctx: &Context) -> Status {
        Status {
            details: "Writing questionable code".to_string(),
            state: ctx.window_title.clone(),
            small_image: Some("vscode".to_string()),
        }
    }
}
