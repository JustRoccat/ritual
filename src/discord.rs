use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{bail, Result};
use serde_json::json;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::UnixStream,
};

use crate::mapper::Status;

const OPCODE_HANDSHAKE: u32 = 0;
const OPCODE_FRAME: u32 = 1;

pub struct DiscordRpc {
    socket_path: String,
    client_id: String,
    pid: u32,
}

impl DiscordRpc {
    pub fn new(socket_path: String, client_id: String, pid: u32) -> Self {
        Self {
            socket_path,
            client_id,
            pid,
        }
    }

    pub async fn publish(&self, status: &Status) -> Result<()> {
        if self.client_id == "YOUR_DISCORD_APP_ID" || self.client_id.trim().is_empty() {
            bail!("discord_client_id is not configured")
        }

        let mut stream = UnixStream::connect(&self.socket_path).await?;
        self.send_packet(
            &mut stream,
            OPCODE_HANDSHAKE,
            &json!({"v": 1, "client_id": self.client_id}),
        )
        .await?;
        let _ = Self::read_packet(&mut stream).await;

        let nonce = format!(
            "ritual-{}",
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis()
        );

        let payload = json!({
            "cmd": "SET_ACTIVITY",
            "args": {
                "pid": self.pid,
                "activity": {
                    "details": status.details,
                    "state": status.state,
                    "assets": {
                        "small_image": status.small_image,
                    }
                }
            },
            "nonce": nonce
        });

        self.send_packet(&mut stream, OPCODE_FRAME, &payload)
            .await?;
        let _ = Self::read_packet(&mut stream).await;
        Ok(())
    }

    async fn send_packet(
        &self,
        stream: &mut UnixStream,
        opcode: u32,
        payload: &serde_json::Value,
    ) -> Result<()> {
        let body = payload.to_string();
        let body_len = u32::try_from(body.len())?;

        stream.write_all(&opcode.to_le_bytes()).await?;
        stream.write_all(&body_len.to_le_bytes()).await?;
        stream.write_all(body.as_bytes()).await?;
        Ok(())
    }

    async fn read_packet(stream: &mut UnixStream) -> Result<(u32, Vec<u8>)> {
        let mut header = [0_u8; 8];
        stream.read_exact(&mut header).await?;

        let opcode = u32::from_le_bytes([header[0], header[1], header[2], header[3]]);
        let body_len = u32::from_le_bytes([header[4], header[5], header[6], header[7]]) as usize;

        let mut body = vec![0_u8; body_len];
        stream.read_exact(&mut body).await?;
        Ok((opcode, body))
    }
}
