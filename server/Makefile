# .PHONY: all build clean re build_from_artifacts release
# .PHONY: docker_build docker_login docker_release

# DIST_DIR = dist
# NAME := server
# VERSION := $(shell cat VERSION.txt | tr -d '\n')
# DOCKER_IMAGE = registry.gitlab.com/bloom42/$(NAME)
# COMMIT = $(shell git rev-parse HEAD)
# DATE := $(shell date +"%Y-%m-%d")

# all: build

# build:
# 	make -C server build
# 	mkdir -p $(DIST_DIR)
# 	cp -r server/dist/* $(DIST_DIR)
# 	make -C webapp build
# 	cp -r webapp/dist $(DIST_DIR)/public

# build_from_artifacts:
# 	mkdir -p $(DIST_DIR)
# 	cp -r server/dist/* $(DIST_DIR)
# 	cp -r webapp/dist $(DIST_DIR)/public

# clean:
# 	rm -rf $(DIST_DIR)
# 	make -C server clean
# 	make -C webapp clean

# re: clean build



.PHONY: build clean re dev test build_static
.PHONY: disposable_emails lint fmt fmt_check lint assets

DIST_DIR = dist
NAME := bloom
VERSION := $(shell cat Cargo.toml | grep '^version =' | cut -d '"' -f2)
DOCKER_IMAGE = registry.gitlab.com/bloom42/bloom/server
COMMIT = $(shell git rev-parse HEAD)
DATE := $(shell date +"%Y-%m-%d")


all: build

build:
	mkdir -p $(DIST_DIR)
	cargo build --release
	cp target/release/bloom_server $(DIST_DIR)/$(NAME)
	cp -r assets $(DIST_DIR)/

build_debug:
	mkdir -p $(DIST_DIR)
	cargo build
	cp target/debug/bloom_server $(DIST_DIR)/$(NAME)
	cp -r assets $(DIST_DIR)/

build_static:
	mkdir -p $(DIST_DIR)
	cargo build --release --target=x86_64-unknown-linux-musl
	cp target/x86_64-unknown-linux-musl/release/bloom_server $(DIST_DIR)/$(NAME)
	cp -r assets $(DIST_DIR)/

dev:
	cargo watch -x 'check'

clean:
	rm -rf $(DIST_DIR) target/

re: clean build

release:
	git tag v$(VERSION)
	git push origin v$(VERSION)

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


crates_login:
	cargo login ${CRATES_TOKEN}

crates_publish:
	cargo publish

disposable_emails:
	cd scripts && ./disposable_emails.sh


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

assets:
	rm -rf assets
	cp -r ../assets/server assets/
