.PHONY: all build clean re build_from_artifacts
.PHONY: docker_build docker_login docker_release

DIST_DIR = dist
NAME := bloom
VERSION := $(shell cat VERSION.txt | tr -d '\n')
DOCKER_IMAGE = registry.gitlab.com/bloom42/$(NAME)
COMMIT = $(shell git rev-parse HEAD)

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


docker_build:
	docker build -t $(DOCKER_IMAGE):latest .
	docker tag $(DOCKER_IMAGE):latest $(DOCKER_IMAGE):$(VERSION)

docker_login:
	docker login -u gitlab-ci-token -p ${CI_JOB_TOKEN} ${CI_REGISTRY}

docker_release:
	docker push $(DOCKER_IMAGE):$(VERSION)
	docker push $(DOCKER_IMAGE):latest
