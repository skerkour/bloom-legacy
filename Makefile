.PHONY: all build clean re build_from_artifacts release
.PHONY: docker_build docker_login docker_release

DIST_DIR = dist
NAME := server
VERSION := $(shell cat VERSION.txt | tr -d '\n')
DOCKER_IMAGE = registry.gitlab.com/bloom42/bloom/legacy
COMMIT = $(shell git rev-parse HEAD)
DATE := $(shell date +"%Y-%m-%d")

all: build

build:
	make -C server build
	mkdir -p $(DIST_DIR)
	cp -r server/dist/* $(DIST_DIR)
	make -C webapp build
	cp -r webapp/dist $(DIST_DIR)/public

build_from_artifacts:
	mkdir -p $(DIST_DIR)
	cp -r server/dist/* $(DIST_DIR)
	cp -r webapp/dist $(DIST_DIR)/public

clean:
	rm -rf $(DIST_DIR)
	make -C server clean
	make -C webapp clean

re: clean build

release:
	git tag legacy/v$(VERSION)
	git push origin legacy/v$(VERSION)



docker_build:
	docker build -t $(DOCKER_IMAGE):v$(VERSION) .
	docker tag $(DOCKER_IMAGE):v$(VERSION) $(DOCKER_IMAGE):latest

docker_login:
	docker login -u gitlab-ci-token -p ${CI_JOB_TOKEN} ${CI_REGISTRY}

docker_release:
	docker push $(DOCKER_IMAGE):v$(VERSION)
	docker push $(DOCKER_IMAGE):latest
