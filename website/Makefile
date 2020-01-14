.PHONY: build clean re install lint test release dev
.PHONY: docker_build docker_login docker_release docker_release_nightly

NAME = website
VERSION = $(shell cat package.json  | grep version | cut -d '"' -f4)
COMMIT = $(shell git rev-parse HEAD)
DOCKER_IMAGE = registry.gitlab.com/bloom42/$(NAME)
DATE = $(shell date +"%Y-%m-%d")

build:
	npm run build
	cp dist/index.html dist/404.html

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

release:
	git tag v$(VERSION)
	git push origin v$(VERSION)

docker_build:
	docker build -t $(DOCKER_IMAGE):latest .
	docker tag $(DOCKER_IMAGE):latest $(DOCKER_IMAGE):$(VERSION)

docker_login:
	docker login -u gitlab-ci-token -p ${CI_JOB_TOKEN} ${CI_REGISTRY}

docker_release:
	docker push $(DOCKER_IMAGE):$(VERSION)
	docker push $(DOCKER_IMAGE):latest

docker_release_nightly:
	docker tag $(DOCKER_IMAGE):latest $(DOCKER_IMAGE):nightly-$(DATE)
	docker push $(DOCKER_IMAGE):nightly-$(DATE)
