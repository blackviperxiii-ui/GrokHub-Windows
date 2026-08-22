//! Grok Build ACP client. The cabin talks to `grok agent stdio`, not the TUI.

mod client;
mod install;
mod locate;
pub mod protocol;

pub use client::{
    connect, discover_session_files, discover_session_files_in, ensure_session_cwd,
    explain_handshake_error, inspect_json, is_session_cwd_error, jsonrpc_error_text, list_sessions,
    merge_grok_sessions, parse_session_list, parse_session_markdown, show_session, split_session_row,
    wait_event, AcpHandle, GrokSession, SpawnOpts,
};
pub use install::{
    begin_grok_install, grok_cli_install_cmd, install_grok_blocking, prepend_dir_to_path,
    prepend_grok_bin_to_process_path,
};
pub use locate::{
    agent_args, agent_args_resume, cabin_grok_home, cabin_leader_socket, doctor_grok_line,
    doctor_line_busy, doctor_missing_hint, find_grok, grok_auth_path, grok_cli_key, grok_home,
    grok_stdout, grok_stdout_timeout, grok_version, hide_windows_console, invalidate_grok_bin_cache,
    parse_grok_auth_key, prepare_cabin_grok_home, which,
};
pub use protocol::{
    merge_tool_card, AcpEvent, PermissionAsk, PermissionMode, SessionMode, ToolCard,
    PROTOCOL_VERSION,
};
