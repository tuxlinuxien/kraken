all:
	cargo build --release

test:
	cargo test

build-cli:
	cargo build -p kraken-cli --release