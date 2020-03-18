DIST_DIR = dist
NAME := bitflow
VERSION := $(shell cat version/version.go| grep "\sVersion =" | cut -d '"' -f2)
COMMIT = $(shell git rev-parse HEAD)
DATE := $(shell date +"%Y-%m-%d")
GO_PKG = $(shell cat go.mod | grep module | cut -d' ' -f2)
DOCKER_IMAGE = registry.$(GO_PKG)
DEFAULT_CONFIG_FILE = bitflow.default.sane
DIST_CONFIG_FILE = bitflow.sane

.PHONY: all
all: build

.PHONY: build
build:
	mkdir -p $(DIST_DIR)
	go build -o $(NAME)
	cp $(NAME) $(DIST_DIR)/$(NAME)
	cp $(DEFAULT_CONFIG_FILE) $(DIST_DIR)/$(DIST_CONFIG_FILE)

.PHONY: build_static
build_static:
	CGO_ENABLED=0 go build -o dist/$(NAME) -a -ldflags "-extldflags "-static" -w -s"
	cp $(DEFAULT_CONFIG_FILE) $(DIST_DIR)/$(DIST_CONFIG_FILE)


.PHONY: dev
dev:
	gowatch -build="go build" -command="./bitflow"

.PHONY: clean
clean:
	rm -rf $(DIST_DIR)

.PHONY: re
re: clean build

.PHONY: test
test:
	go vet -all ./...
	go test -v -race -covermode=atomic ./...


.PHONY: mod_install
mod_install:
	go mod install


.PHONY: mod_verify
mod_verify:
	go mod verify


.PHONY: tidy
tidy:
	go mod tidy


.PHONY: fmt
fmt:
	go fmt ./...


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


.PHONY: docker
docker:
	docker build -t $(DOCKER_IMAGE):latest .

.PHONY: docker_login
docker_login:
	docker login -u gitlab-ci-token -p ${CI_JOB_TOKEN} ${CI_REGISTRY}

.PHONY: docker_release
docker_release:
	docker push $(DOCKER_IMAGE):latest
