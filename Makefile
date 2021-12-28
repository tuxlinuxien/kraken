all:
	cargo build --release

build-cli:
	cargo build -p kraken-cli --release