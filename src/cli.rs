pub(crate) use clap::Parser;

#[derive(Parser)]
#[command(author, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<CliCommand>,

    /// Attaches to this target when no command is specified.
    pub attach_target: Option<String>,
}

#[derive(clap::Subcommand)]
pub enum CliCommand {
    /// List tmux sessions.
    #[command(alias = "ls")]
    List,

    /// Detaches from the current tmux session.
    #[command(alias = "d")]
    Detach,

    /// Kills a tmux session. Specify a target or pick one with fzf.
    #[command(alias = "k")]
    Kill { target_session: Option<String> },
}
