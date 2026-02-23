use anyhow::Result;
use serde_json::json;
use tokio::{io::AsyncWriteExt, net::UnixStream};

use crate::mapper::Status;

pub struct DiscordRpc {
    socket_path: String,
}

impl DiscordRpc {
    pub fn new(socket_path: String) -> Self {
        Self { socket_path }
    }

    pub async fn publish(&self, status: &Status) -> Result<()> {
        let mut stream = UnixStream::connect(&self.socket_path).await?;
        let payload = json!({
            "cmd": "SET_ACTIVITY",
            "args": {
                "activity": {
                    "details": status.details,
                    "state": status.state,
                    "assets": {
                        "small_image": status.small_image,
                    }
                }
            },
            "nonce": "ritual"
        });
        stream.write_all(payload.to_string().as_bytes()).await?;
        Ok(())
    }
}
