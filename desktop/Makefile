.PHONY: dev install clean assets start
.PHONY: test lint build dev_native


dev_vue:
	npm run dev:vue

dev_native:
	npm run dev:native

start:
	npm urn start

clean:
	rm -rf dist node_modules/

install:
	npm install

test:
	npm run test:unit

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
