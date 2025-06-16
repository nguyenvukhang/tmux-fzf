# tmux-fzf

This project is fully functional.

Install it with `make install` (this requires `cargo`), and enjoy.

```
Usage: tm [ATTACH_TARGET] [COMMAND]

Commands:
  list    List tmux sessions
  detach  Detaches from the current tmux session
  kill    Kills a tmux session. Specify a target or pick one with fzf
  help    Print this message or the help of the given subcommand(s)

Arguments:
  [ATTACH_TARGET]  Attaches to this target when no command is specified

Options:
  -h, --help  Print help
```
