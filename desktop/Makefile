.PHONY: build
build:
	npm run build:native
	npm run build:electron

.PHONY: clean
clean:
	rm -rf dist dist_electron

.PHONY: re
re: clean build

.PHONY: dev
dev:
	npm urn dev:electron

.PHONY: install
install:
	npm install

.PHONY: test
test:
	npm run test:unit

.PHONY: lint
lint:
	npm run lint

.PHONY: assets
assets:
	rm -rf public/assets
	mkdir -p public/assets public/assets/fonts public/assets/imgs
	cp -r ../assets/icons public/assets/imgs/icons
	cp ../assets/desktop/icons/* public/assets/imgs/icons/
	cp ../assets/desktop/imgs/* public/assets/imgs/
	cp ../assets/fonts/rounded_elegance.ttf public/assets/fonts/

.PHONY: icons
icons:
	npm run icons
