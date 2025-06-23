test:
	tmux ls -F "#{session_name}: #{session_windows} windows (created #{t:session_created}) #{?session_attached, (attached),}"

build:
	cargo build

run:
	cargo run

s: setup
setup:
	@tmux new -ds dank
	@tmux new -ds meme
	tmux ls

install:
	cargo install --locked --all-features --force --path .
