use crate::detector::Context;
use crate::mapper::Status;

use super::Plugin;

pub struct GenericPlugin;

impl Plugin for GenericPlugin {
    fn supports(&self, _ctx: &Context) -> bool {
        true
    }

    fn generate(&self, ctx: &Context) -> Status {
        Status {
            details: format!("Using {}", ctx.app_name),
            state: ctx.window_title.clone(),
            small_image: Some("generic".to_string()),
        }
    }
}
