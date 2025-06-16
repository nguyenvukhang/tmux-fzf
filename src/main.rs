mod cli;
mod tmux;

use cli::{Cli, CliCommand, Parser};
use tmux::send;

use std::io;
use std::io::Write;
use std::process::{Child, Command, ExitCode, Stdio};

fn spawn_fzf() -> io::Result<Child> {
    Command::new("fzf")
        .arg("-0")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
}

fn handle_command(command: CliCommand) -> io::Result<ExitCode> {
    match command {
        CliCommand::List => send(tmux::ls()),
        CliCommand::Detach => tmux::detach(),
        CliCommand::Kill { target_session } => {
            if let Some(v) = target_session {
                return tmux::kill(&v);
            }
            let tmux_ls = tmux::ls().stdout(Stdio::piped()).output()?;
            let mut fzf = spawn_fzf()?;
            fzf.stdin.as_mut().unwrap().write_all(&tmux_ls.stdout)?;
            let fzf = fzf.wait_with_output()?;
            if !fzf.status.success() {
                return Ok(ExitCode::FAILURE); // Nothing is selected with fzf.
            };
            let selected_line = core::str::from_utf8(&fzf.stdout).unwrap();
            let target_session = selected_line.split_once(':').unwrap().0;
            tmux::kill(target_session)
        }
    }
}

fn handle_empty() -> io::Result<ExitCode> {
    let tmux_ls = tmux::ls().stdout(Stdio::piped()).output()?;
    if tmux_ls.stdout.is_empty() {
        return send(tmux::new_session("0"));
    }
    let mut fzf = spawn_fzf()?;
    fzf.stdin.as_mut().unwrap().write_all(&tmux_ls.stdout)?;
    let fzf = fzf.wait_with_output()?;
    if !fzf.status.success() {
        return Ok(ExitCode::FAILURE); // Nothing is selected with fzf.
    };
    let selected_line = core::str::from_utf8(&fzf.stdout).unwrap();
    let target_session = selected_line.split_once(':').unwrap().0;
    tmux::attach(target_session)
}

fn main() -> io::Result<ExitCode> {
    let cli = Cli::parse();
    if let Some(command) = cli.command {
        return handle_command(command);
    }
    let Some(target) = cli.attach_target else { return handle_empty() };
    if !tmux::has_session(&target)? {
        tmux::new_detached_session(&target).spawn()?.wait()?;
    }
    tmux::attach(&target)
}
