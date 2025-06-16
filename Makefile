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
