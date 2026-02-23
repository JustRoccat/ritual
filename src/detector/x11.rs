use anyhow::{anyhow, Context as _, Result};
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{AtomEnum, ConnectionExt};
use x11rb::rust_connection::RustConnection;

use super::Context;

pub struct X11Detector {
    conn: RustConnection,
    root: u32,
}

impl X11Detector {
    pub fn new() -> Result<Self> {
        let (conn, screen_num) = RustConnection::connect(None).context("cannot connect to X11")?;
        let root = conn.setup().roots[screen_num].root;
        Ok(Self { conn, root })
    }

    pub fn active_context(&self) -> Result<Context> {
        let active_window_atom = self.atom("_NET_ACTIVE_WINDOW")?;
        let window_name_atom = self.atom("_NET_WM_NAME")?;
        let window_pid_atom = self.atom("_NET_WM_PID")?;

        let active = self
            .conn
            .get_property(false, self.root, active_window_atom, AtomEnum::WINDOW, 0, 1)?
            .reply()?
            .value32()
            .and_then(|mut v| v.next())
            .ok_or_else(|| anyhow!("no active window"))?;

        let title = self
            .conn
            .get_property(false, active, window_name_atom, AtomEnum::STRING, 0, 1024)?
            .and_then(|c| c.reply())
            .ok()
            .map(|reply| String::from_utf8_lossy(&reply.value).to_string())
            .unwrap_or_else(|| "unknown window".to_string());

        let pid = self
            .conn
            .get_property(false, active, window_pid_atom, AtomEnum::CARDINAL, 0, 1)?
            .reply()?
            .value32()
            .and_then(|mut v| v.next());

        let app_name = pid
            .and_then(crate::detector::processes::process_name_for_pid)
            .unwrap_or_else(|| title.clone());

        Ok(Context {
            app_name: normalize_app_name(&app_name),
            window_title: title,
            pid,
        })
    }

    fn atom(&self, name: &str) -> Result<u32> {
        Ok(self.conn.intern_atom(false, name.as_bytes())?.reply()?.atom)
    }
}

fn normalize_app_name(name: &str) -> String {
    name.to_lowercase().replace(' ', "_")
}
