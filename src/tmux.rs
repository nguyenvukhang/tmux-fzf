//! Common tmux routines.

use std::io;
use std::process::{Command, ExitCode};

macro_rules! tmux{($($arg:expr),+)=>{{let mut z=std::process::Command::new("tmux");$(z.arg($arg);)*z}}}

/// List sessions.
pub fn ls() -> Command {
    tmux!("ls")
}

/// Create a new session.
pub fn new_session(name: &str) -> Command {
    tmux!("new", "-s", name)
}

/// Create a new session, detached.
pub fn new_detached_session(name: &str) -> Command {
    tmux!("new", "-ds", name)
}

pub fn send(mut cmd: Command) -> io::Result<ExitCode> {
    let status = cmd.spawn()?.wait()?;
    if status.success() {
        return Ok(ExitCode::SUCCESS);
    }
    Ok(ExitCode::from(u8::try_from(status.code().unwrap_or(1)).unwrap_or(1)))
}

/// Attach to a target session. Uses `exec` on unix systems.
pub fn attach(session: &str) -> io::Result<ExitCode> {
    if cfg!(unix) {
        use std::os::unix::process::CommandExt;
        Err(tmux!("a", "-t", session).exec())
    } else {
        send(tmux!("a", "-t", session))
    }
}

/// Check if tmux has a particular session running.
pub fn has_session(session: &str) -> io::Result<bool> {
    Ok(tmux!("has", "-t", session).output()?.status.success())
}

/// Kills a particular session.
pub fn kill(session: &str) -> io::Result<ExitCode> {
    send(tmux!("kill-session", "-t", session))
}

/// Detach from tmux server.
pub fn detach() -> io::Result<ExitCode> {
    send(tmux!("detach"))
}
