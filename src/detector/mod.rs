pub mod processes;
pub mod x11;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Context {
    pub app_name: String,
    pub window_title: String,
    pub pid: Option<u32>,
}

impl Context {
    pub fn identity(&self) -> String {
        format!("{}::{}", self.app_name, self.window_title)
    }
}
