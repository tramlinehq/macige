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
	cargo clippy -- -D warnings

.PHONY: check
check:
	cargo check

.PHONY: fmt
fmt:
	cargo fmt --all

.PHONY: clean
clean:
	cargo clean

.PHONY: all
all: check test fmt clippy build
