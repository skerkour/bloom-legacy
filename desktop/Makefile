.PHONY: dev install clean
.PHONY: test lint electron_build vue_build


dev:
	npm run serve

clean:
	rm -rf dist node_modules/

install:
	npm install

test:
	npm run test:unit

lint:
	npm run lint

electron_build: vue_build

vue_build:
	npm run build
