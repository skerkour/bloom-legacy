.PHONY: dev install clean assets dev_vue dev_electron icons
.PHONY: test lint build dev_native test_native lint_native

build:
	npm run build:native
	npm run build:electron

clean:
	rm -rf dist dist_electron

re: clean build

dev_native:
	npm run dev:native

dev_electron:
	npm urn dev:electron

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

assets:
	rm -rf public/assets
	mkdir -p public/assets public/assets/fonts public/assets/imgs
	cp -r ../assets/icons public/assets/imgs/icons
	cp ../assets/desktop/icons/* public/assets/imgs/icons/
	cp ../assets/desktop/imgs/* public/assets/imgs/
	cp ../assets/fonts/rounded_elegance.ttf public/assets/fonts/

icons:
	npm run icons
