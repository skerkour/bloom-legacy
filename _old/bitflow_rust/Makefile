.PHONY: build clean re dev test build_static release fmt fmt_check lint
.PHONY: docker_login docker_build docker_release dev_docker_build dev_docker_run docker_release_nightly

DIST_DIR = dist
NAME := $(shell cat Cargo.toml | grep "^name\s=" | cut -d '"' -f2)
VERSION := $(shell cat Cargo.toml | grep "^version\s=" | cut -d '"' -f2)
DOCKER_IMAGE = registry.gitlab.com/bloom42/bloom/$(NAME)
COMMIT = $(shell git rev-parse HEAD)
DATE := $(shell date +"%Y-%m-%d")


all: build

build:
	mkdir -p $(DIST_DIR)
	cargo build --release
	cp target/release/$(NAME) $(DIST_DIR)/$(NAME)

build_debug:
	mkdir -p $(DIST_DIR)
	cargo build
	cp target/debug/$(NAME) $(DIST_DIR)/$(NAME)

build_static:
	mkdir -p $(DIST_DIR)
	cargo build -p $(NAME) --release --target=x86_64-unknown-linux-musl
	cp target/x86_64-unknown-linux-musl/release/$(NAME) $(DIST_DIR)/$(NAME)

dev:
	cargo watch -x 'run -- worker'

clean:
	rm -rf $(DIST_DIR) target/

re: clean build

test:
	cargo test

fmt:
	cargo fmt

fmt_check:
	cargo fmt --all -- --check

lint:
	cargo clippy -- -D warnings -A clippy::needless_return

audit:
	cargo audit

release:
	git tag v$(VERSION)
	git push origin v$(VERSION)

crates_login:
	cargo login ${CRATES_TOKEN}

crates_publish:
	cargo publish


dev_docker_build:
	docker build -t $(DOCKER_IMAGE)_dev:latest -f dev.Dockerfile .

dev_docker_run:
	docker run -ti -v `pwd`:/bitflow $(DOCKER_IMAGE)_dev:latest


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
