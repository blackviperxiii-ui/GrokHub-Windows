//! LAN hub — same /v1 contract as grokhub --hub.

use grokhub_core::DEFAULT_PORT;
use std::env;
use std::path::PathBuf;

fn persist_path() -> PathBuf {
    if let Ok(p) = env::var("GROKHUB_CONFIG") {
        return PathBuf::from(p).join("hub-state.json");
    }
    if cfg!(windows) {
        if let Ok(app) = env::var("APPDATA") {
            if !app.trim().is_empty() {
                return PathBuf::from(app).join("GrokHub").join("hub-state.json");
            }
        }
    }
    if let Some(home) = grokhub_core::user_home() {
        return home.join(".config").join("GrokHub").join("hub-state.json");
    }
    PathBuf::from("hub-state.json")
}

fn main() {
    let port = env::var("GROKHUB_HUB_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_PORT);
    let banner = format!("grokhub-hub {}", env!("CARGO_PKG_VERSION"));
    if let Err(e) = grokhub_hub::run(port, persist_path(), &banner) {
        eprintln!("hub failed: {e}");
        std::process::exit(1);
    }
}
