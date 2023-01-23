.PHONY: trunk
trunk:
	cargo install --locked trunk

.PHONY: serve
serve:
	cargo build
	trunk serve --open --address 0.0.0.0

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
