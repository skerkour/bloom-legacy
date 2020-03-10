DIST_DIR = dist
NAME := bitflow
VERSION := $(shell cat version/version.go| grep "\sVersion =" | cut -d '"' -f2)
DOCKER_IMAGE = registry.gitlab.com/bloom42/$(NAME)
COMMIT = $(shell git rev-parse HEAD)
DATE := $(shell date +"%Y-%m-%d")

.PHONY: all
all: build

.PHONY: build
build:
	mkdir -p $(DIST_DIR)
	go build -mod=vendor -o $(NAME)
	cp $(NAME) $(DIST_DIR)/$(NAME)
	cp bitflow.default.sane $(DIST_DIR)/bitflow.sane

.PHONY: dev
dev:
	gowatch -build="go build -mod=vendor" -command="./bitflow"

.PHONY: clean
clean:
	rm -rf $(DIST_DIR)

.PHONY: re
re: clean build

.PHONY: test
test:
	go vet -all -mod=vendor ./...
	go test -v -race -covermode=atomic -mod=vendor ./...


.PHONY: install
install:
	go install -mod=vendor


.PHONY: fmt
fmt:
	go fmt

.PHONY: fmt_check
fmt_check:
	test -z $$(gofmt -l $$(find . -name '*.go' -type f -print | grep -v /vendor/))

.PHONY: lint
lint:
	# test -z $$(gofmt -l $$(find . -name '*.go' -type f -print | grep -v /vendor/))

.PHONY: release
release:
	git checkout release
	git merge master
	git push
	git tag v$(VERSION)
	git push origin v$(VERSION)
	git checkout master


# dev_docker_build:
# 	docker build -t $(DOCKER_IMAGE)_dev:latest -f dev.Dockerfile .

# dev_docker_run:
# 	docker run -ti -v `pwd`:/bitflow $(DOCKER_IMAGE)_dev:latest

.PHONY: docker_build
docker_build:
	docker build -t $(DOCKER_IMAGE):latest .
	docker tag $(DOCKER_IMAGE):latest $(DOCKER_IMAGE):$(VERSION)

.PHONY: docker_login
docker_login:
	docker login -u gitlab-ci-token -p ${CI_JOB_TOKEN} ${CI_REGISTRY}

.PHONY: docker_release
docker_release:
	docker push $(DOCKER_IMAGE):$(VERSION)
	docker push $(DOCKER_IMAGE):latest

# docker_release_nightly:
# 	docker tag $(DOCKER_IMAGE):latest $(DOCKER_IMAGE):nightly-$(DATE)
# 	docker push $(DOCKER_IMAGE):nightly-$(DATE)
