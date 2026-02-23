pub mod generic;
pub mod steam;
pub mod vscode;

use crate::detector::Context;
use crate::mapper::Status;

pub trait Plugin: Send + Sync {
    fn supports(&self, ctx: &Context) -> bool;
    fn generate(&self, ctx: &Context) -> Status;
}
