.PHONY: dev install clean
.PHONY: electron_test vue_lint electron_build vue_build


dev:
	npm run serve

clean:
	rm -rf dist node_modules/

install:
	npm install

electron_test: vue_lint
	npm run test:unit

vue_lint:
	npm run lint

electron_build: vue_build

vue_build:
	npm run build
