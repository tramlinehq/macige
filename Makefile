.PHONY: trunk
trunk:
	cargo install --locked trunk

.PHONY: wasm
wasm:
	rustup target add wasm32-unknown-unknown

.PHONY: serve
serve:
	cargo build
	trunk serve --open --address 127.0.0.1

.PHONY: build
build:
	rm -rfv dist
	cargo build --release
	trunk build --release

.PHONY: clippy
clippy:
	cargo clippy

.PHONY: check
check:
	cargo check

.PHONY: fmt
fmt:
	cargo fmt --all --check

.PHONY: clean
clean:
	cargo clean

.PHONY: release
release: trunk build

.PHONY: all
all: check test fmt clippy build
