.PHONY: build clean re dev test build_static release
.PHONY: docker_login docker_build docker_release docker_push docker_dev
.PHONY: disposable_emails

DIST_DIR = dist
NAME := $(shell cat bloom/kernel/Cargo.toml | grep "^name\s=" | cut -d '"' -f2)
VERSION := $(shell cat bloom/kernel/Cargo.toml | grep "^version\s=" | cut -d '"' -f2)
DOCKER_IMAGE = quay.io/bloom42/$(NAME)
COMMIT = $(shell git rev-parse HEAD)

all: build

build:
	mkdir -p $(DIST_DIR)
	cargo build -p api --release -j 1
	cp target/release/api $(DIST_DIR)/$(NAME)
	cp -r assets $(DIST_DIR)/

build_debug:
	mkdir -p $(DIST_DIR)
	cargo build -p api
	cp target/debug/api $(DIST_DIR)/$(NAME)
	cp -r assets $(DIST_DIR)/

build_static:
	mkdir -p $(DIST_DIR)
	cargo build -p api --release --target=x86_64-unknown-linux-musl
	cp target/x86_64-unknown-linux-musl/release/api $(DIST_DIR)/$(NAME)
	cp -r assets $(DIST_DIR)/

dev:
	cargo watch -x 'run -p api'

clean:
	rm -rf $(DIST_DIR) target/

re: clean build

test:
	cargo test

lint:
	cargo +nightly fmt
	cargo clippy

audit:
	cargo audit

release:
	git tag v$(VERSION)
	git push origin v$(VERSION)

crates_login:
	cargo login ${CRATES_TOKEN}

crates_publish:
	cargo publish

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

docker_dev:
	docker build -t $(DOCKER_IMAGE)_dev:latest -f dev.Dockerfile .

disposable_emails:
	cd scripts && ./disposable_emails.sh
