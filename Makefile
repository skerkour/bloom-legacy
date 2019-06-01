.PHONY: all build clean re
.PHONY: docker_build docker_login docker_push docker_release

DIST_DIR = dist
NAME := bloom
VERSION := $(shell cat server/kernel/Cargo.toml | grep "^version\s=" | cut -d '"' -f2)
DOCKER_IMAGE = registry.gitlab.com/bloom42/$(NAME)
COMMIT = $(shell git rev-parse HEAD)

all: build

build:
	make -C server build
	mkdir -p $(DIST_DIR)
	cp -r server/dist/* $(DIST_DIR)
	make -C webapp build
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
	echo ${DOCKER_PASSWORD} | docker login -u ${DOCKER_USERNAME} --password-stdin ${DOCKER_REGISTRY}

docker_push:
	docker push $(DOCKER_IMAGE):latest

docker_release:
	docker push $(DOCKER_IMAGE):$(VERSION)
	docker push $(DOCKER_IMAGE):latest
