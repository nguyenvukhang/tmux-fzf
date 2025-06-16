mod cli;

use cli::{Cli, CliCommand, Parser};

use std::io;
use std::io::Write;
use std::process::{Child, Command, ExitCode, ExitStatus, Stdio};

macro_rules! tmux{($($arg:expr),+)=>{{let mut z=Command::new("tmux");$(z.arg($arg);)*z}}}

fn has_session(session: &str) -> io::Result<bool> {
    Ok(tmux!("has", "-t", session).output()?.status.success())
}

fn run(mut cmd: Command) -> io::Result<ExitStatus> {
    cmd.spawn()?.wait()
}

fn send(mut cmd: Command) -> io::Result<ExitCode> {
    let status = cmd.spawn()?.wait()?;
    if status.success() {
        return Ok(ExitCode::SUCCESS);
    }
    Ok(ExitCode::from(u8::try_from(status.code().unwrap_or(1)).unwrap_or(1)))
}

fn spawn_fzf() -> io::Result<Child> {
    Command::new("fzf")
        .arg("-0")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
}

fn handle_command(command: CliCommand) -> io::Result<ExitCode> {
    match command {
        CliCommand::List => send(tmux!("ls")),
        CliCommand::Detach => send(tmux!("detach")),
        CliCommand::Kill { target_session } => {
            if let Some(v) = target_session {
                return send(tmux!("kill-session", "-t", v));
            }
            let tmux_ls = tmux!("ls").stdout(Stdio::piped()).output()?;
            let mut fzf = spawn_fzf()?;
            fzf.stdin.as_mut().unwrap().write_all(&tmux_ls.stdout)?;
            let fzf_o = fzf.wait_with_output()?;
            if !fzf_o.status.success() {
                return Ok(ExitCode::SUCCESS); // Nothing is selected with fzf.
            };
            let selected_line = core::str::from_utf8(&fzf_o.stdout).unwrap();
            let target_session = selected_line.split_once(':').unwrap().0;
            send(tmux!("kill-session", "-t", target_session))
        }
    }
}

fn handle_empty() -> io::Result<ExitCode> {
    let tmux_ls = tmux!("ls").stdout(Stdio::piped()).output()?;
    if tmux_ls.stdout.is_empty() {
        return send(tmux!("new", "-s", "0"));
    }
    let mut fzf = spawn_fzf()?;
    fzf.stdin.as_mut().unwrap().write_all(&tmux_ls.stdout)?;
    let fzf_o = fzf.wait_with_output()?;
    if !fzf_o.status.success() {
        return Ok(ExitCode::SUCCESS); // Nothing is selected with fzf.
    };
    let selected_line = core::str::from_utf8(&fzf_o.stdout).unwrap();
    let target_session = selected_line.split_once(':').unwrap().0;
    send(tmux!("a", "-t", target_session))
}

fn main() -> io::Result<ExitCode> {
    let cli = Cli::parse();
    if let Some(command) = cli.command {
        return handle_command(command);
    }
    let Some(target) = cli.attach_target else { return handle_empty() };
    if !has_session(&target)? {
        run(tmux!("new", "-ds", &target))?;
    }
    send(tmux!("a", "-t", target))
}
