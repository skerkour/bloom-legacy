.PHONY: dev fmt fmt_check lint test

dev:
	cargo watch -s 'neon build -p ..'

build:
	neon build -p .. --release

build_debug:
	neon build -p ..

test:
	cargo test

fmt:
	cargo fmt

fmt_check:
	cargo fmt --all -- --check

lint:
	cargo clippy -- -D warnings -A clippy::needless_return
