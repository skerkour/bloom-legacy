.PHONY: build clean re install lint test release
.PHONY: dev staging deploy production

VERSION := $(shell cat package.json  | grep version | cut -d '"' -f4)
COMMIT = $(shell git rev-parse HEAD)

build:
	npm run build

dev:
	npm run serve

release:
	git tag v$(VERSION)
	git push origin v$(VERSION)


build_staging:
	NODE_ENV=satging npm run build:staging

clean:
	rm -rf dist

lint:
	npm run lint

test: lint
	npm run test:unit

re: clean build

deploy:
	firebase deploy

deploy_staging:
	firebase deploy -P staging

production: build deploy

staging: build_staging deploy_staging
