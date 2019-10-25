.PHONY: dev install clean assets dev_vue dev_electron
.PHONY: test lint build dev_native test_native lint_native


dev_vue:
	npm run dev:vue

dev_native:
	npm run dev:native

dev_electron:
	npm urn dev:electron

clean:
	rm -rf dist node_modules/

install:
	npm install

test:
	npm run test:unit

test_native:
	cd native && cargo test

lint_native:
	cd native && cargo fmt --all -- --check
	cd native && cargo clippy -- -D warnings -A clippy::needless_return

fmt_native:
	cd native && cargo fmt

lint:
	npm run lint

build:
	npm run build

assets:
	rm -rf public/assets
	mkdir -p public/assets public/assets/fonts
	cp -r ../assets/icons public/assets/icons
	cp ../assets/desktop/icons/* public/assets/icons/
	cp ../assets/fonts/rounded_elegance.ttf public/assets/fonts/
