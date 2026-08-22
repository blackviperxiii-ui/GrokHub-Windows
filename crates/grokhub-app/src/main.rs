//! GrokHub native cabin. No Electron. No Tauri.
//!
//! Windows Explorer/Start must not allocate a console. Closing that console
//! kills the cabin. CLI flags attach the parent console when there is one.

#![cfg_attr(not(test), windows_subsystem = "windows")]

mod app;
mod build_agent;
mod helpers;
mod titlebar;
mod cards;
mod icons;
mod theme;
mod cli;
mod config;
mod desktop;
mod github;
mod host;
mod markdown;
mod night;
mod recipes;
mod notify;
mod store;
mod voice_ws;
mod oauth;
mod secrets;
mod skills;
mod threads;
mod tray;
mod window;
mod update;
mod xai;

use app::Cabin;
use cli::{parse_args, Launch};
use eframe::egui;
use grokhub_core::{doctor_lines, doctor_ok, DEFAULT_PORT, HUB_KIND};
use std::env;

fn main() {
    let launch = parse_args(&env::args().collect::<Vec<_>>());
    match launch {
        Launch::Cabin | Launch::Agent => {}
        Launch::Hub => attach_cli_console(true),
        Launch::Version | Launch::Help | Launch::Doctor | Launch::Update | Launch::Oauth => {
            attach_cli_console(false)
        }
    }
    match launch {
        Launch::Version => {
            println!("{}", env!("CARGO_PKG_VERSION"));
        }
        Launch::Help => {
            eprint!(
                "grokhub {} — native cabin\n\n  grokhub           cabin (close stays in the tray)\n  grokhub --agent   cabin in the tray, window hidden\n  grokhub --hub     LAN hub only\n  grokhub --oauth   xAI device-code (Grok)\n  grokhub --update  git pull + install.sh --user\n  grokhub --doctor  auth / memory / hub kind\n  grokhub --version\n",
                env!("CARGO_PKG_VERSION")
            );
        }
        Launch::Doctor => run_doctor(),
        Launch::Oauth => run_oauth_cli(),
        Launch::Update => run_update_cli(),
        Launch::Hub => run_hub(),
        Launch::Agent => {
            if let Err(e) = run_cabin(true) {
                eprintln!("{e}");
                std::process::exit(1);
            }
        }
        Launch::Cabin => {
            if let Err(e) = run_cabin(false) {
                eprintln!("{e}");
                std::process::exit(1);
            }
        }
    }
}

fn run_oauth_cli() {
    match oauth::start_device() {
        Ok(start) => {
            println!("Grok OAuth user code: {}", start.user_code);
            println!("{}", start.verification_uri);
            if let Some(u) = &start.verification_uri_complete {
                println!("{u}");
                let _ = oauth::open_browser(u);
            } else {
                let _ = oauth::open_browser(&start.verification_uri);
            }
            match oauth::poll_until_ready(&start.device_code, start.interval) {
                Ok(tokens) => {
                    let mut s = secrets::load();
                    s.oauth = Some(tokens.clone());
                    if let Err(e) = secrets::save(&s) {
                        eprintln!("{e}");
                        std::process::exit(1);
                    }
                    println!(
                        "connected {}",
                        tokens.email.or(tokens.name).unwrap_or_else(|| "grok".into())
                    );
                }
                Err(e) => {
                    eprintln!("{e}");
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}

fn run_doctor() {
    let cfg = config::load();
    let sec = secrets::load();
    let mem_ok = std::fs::create_dir_all(config::memory_dir()).is_ok();
    let authed = grokhub_core::has_auth(
        secrets::console_key(&sec, &cfg.api_key),
        &secrets::access_token(&sec),
    );
    let mut lines = doctor_lines(authed, mem_ok, HUB_KIND);
    lines.extend(grokhub_core::doctor_extras(None, crate::skills::list_skills().len()));
    for l in &lines {
        println!("{} {}", if l.ok { "ok " } else { "ERR" }, l.text);
    }
    if !doctor_ok(&lines) {
        std::process::exit(1);
    }
}

fn run_update_cli() {
    let cfg = config::load();
    let Some(src) = update::resolve_source(&cfg.source_dir) else {
        eprintln!("no GrokHub source tree — set GROKHUB_SRC or Settings → source");
        std::process::exit(1);
    };
    match update::run_update(&src) {
        Ok(out) => print!("{out}"),
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}

fn run_hub() {
    let port = env::var("GROKHUB_HUB_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_PORT);
    let banner = format!("grokhub {} hub", env!("CARGO_PKG_VERSION"));
    if let Err(e) = grokhub_hub::run(port, config::hub_state_path(), &banner) {
        eprintln!("hub failed: {e}");
        std::process::exit(1);
    }
}

fn run_cabin(hidden: bool) -> eframe::Result<()> {
    if !tray::try_claim_cabin() {
        return Ok(());
    }
    tray::pin_session_bus();
    tray::force_x11_for_close_to_tray(
        env::var_os("DISPLAY").is_some(),
        env::var_os("WAYLAND_DISPLAY").is_some() || env::var_os("WAYLAND_SOCKET").is_some(),
    );
    let geom = window::clamp_geom(config::load().window);
    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size(window::launch_size(&geom))
        .with_min_inner_size([window::WIN_MIN_W, window::WIN_MIN_H])
        .with_title("GrokHub")
        .with_app_id("grokhub")
        .with_decorations(false)
        .with_maximized(geom.maximized)
        .with_visible(!hidden);
    if let Some(pos) = window::launch_pos(&geom) {
        viewport = viewport.with_position(pos);
    }
    let opts = eframe::NativeOptions {
        viewport,
        // eframe window persistence also restores visibility; close-to-tray would come back withdrawn.
        persist_window: false,
        ..Default::default()
    };
    eframe::run_native(
        "GrokHub",
        opts,
        Box::new(move |cc| {
            crate::theme::install_fonts(&cc.egui_ctx);
            Ok(Box::new(Cabin::new(hidden)))
        }),
    )
}

fn attach_cli_console(alloc_if_orphan: bool) {
    #[cfg(windows)]
    {
        win_console::attach(alloc_if_orphan);
    }
    let _ = alloc_if_orphan;
}

#[cfg(windows)]
mod win_console {
    use windows_sys::Win32::System::Console::{
        AllocConsole, AttachConsole, GetStdHandle, ATTACH_PARENT_PROCESS, STD_ERROR_HANDLE,
        STD_OUTPUT_HANDLE,
    };

    #[link(name = "ucrt")]
    extern "C" {
        fn _open_osfhandle(osfhandle: isize, flags: i32) -> i32;
        fn _dup2(fd1: i32, fd2: i32) -> i32;
    }

    const O_TEXT: i32 = 0x4000;

    pub fn attach(alloc_if_orphan: bool) {
        unsafe {
            if AttachConsole(ATTACH_PARENT_PROCESS) == 0 {
                if !alloc_if_orphan {
                    return;
                }
                if AllocConsole() == 0 {
                    return;
                }
            }
            bind_stdio(STD_OUTPUT_HANDLE, 1);
            bind_stdio(STD_ERROR_HANDLE, 2);
        }
    }

    unsafe fn bind_stdio(std_id: u32, fd: i32) {
        let h = GetStdHandle(std_id);
        if h.is_null() || h == (-1isize as _) {
            return;
        }
        let osfh = _open_osfhandle(h as isize, O_TEXT);
        if osfh != -1 {
            let _ = _dup2(osfh, fd);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn windows_cabin_is_a_gui_subsystem() {
        let src = include_str!("main.rs");
        assert!(
            src.contains("windows_subsystem = \"windows\""),
            "Explorer must not attach a console that kills the cabin when closed: {src}"
        );
        assert!(
            src.contains("AttachConsole"),
            "grokhub --version from PowerShell must still print: {src}"
        );
        assert!(
            src.contains("not(test)"),
            "cargo test must keep a console: {src}"
        );
    }
}
