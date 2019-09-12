.PHONY: build clean re install lint test release
.PHONY: dev

VERSION := $(shell cat package.json  | grep version | cut -d '"' -f4)
COMMIT = $(shell git rev-parse HEAD)

build:
	npm run build

build_staging:
	NODE_ENV=satging npm run build:staging


install:
	npm install

dev:
	npm run serve

clean:
	rm -rf dist

lint:
	npm run lint

test:
	npm run test:unit

re: clean build
