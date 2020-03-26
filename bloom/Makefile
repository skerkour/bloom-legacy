NAME = $(shell cat version/version.go| grep "\sName" | cut -d '"' -f2)
VERSION := $(shell cat version/version.go| grep "\sVersion\s=" | cut -d '"' -f2)
COMMIT = $(shell git rev-parse HEAD)
DATE := $(shell date +"%Y-%m-%d")
GO_PKG = $(shell cat go.mod | grep module | cut -d' ' -f2)
DOCKER_IMAGE = registry.gitlab.com/bloom42/bloom
DIST_DIR = dist
DEFAULT_CONFIG_FILE = bloom.default.json
DIST_CONFIG_FILE = bloom.json


.PHONY: all
all: build

.PHONY: fmt
fmt:
	go fmt ./...

.PHONY: test
test:
	go test ./...

.PHONY: dev
dev:
	gowatch -log-prefix=false -build="make build" -command="dist/$(NAME) server run"

.PHONY: build
build:
	go build -o dist/$(NAME) -ldflags "-X $(GO_PKG)/version.GitCommit=$(COMMIT) \
		-X $(GO_PKG)/version.UTCBuildTime=`TZ=UTC date -u '+%Y-%m-%dT%H:%M:%SZ'` \
		-X $(GO_PKG)/version.GoVersion=`go version | cut -d' ' -f 3 | cut -c3-`"
	cp -r migrations $(DIST_DIR)/
	cp $(DEFAULT_CONFIG_FILE) $(DIST_DIR)/$(DIST_CONFIG_FILE)

.PHONY: build_static
build_static:
	CGO_ENABLED=0 go build -o dist/$(NAME) -a -ldflags "-extldflags "-static" -w -s \
		-X $(GO_PKG)/version.GitCommit=$(COMMIT) \
		-X $(GO_PKG)/version.UTCBuildTime=`TZ=UTC date -u '+%Y-%m-%dT%H:%M:%SZ'` \
		-X $(GO_PKG)/version.GoVersion=`go version | cut -d' ' -f 3 | cut -c3-`"
	cp -r migrations $(DIST_DIR)/
	cp $(DEFAULT_CONFIG_FILE) $(DIST_DIR)/$(DIST_CONFIG_FILE)


.PHONY: clean
clean:
	rm -rf dist


.PHONY: gqlgen
gqlgen:
	go run github.com/99designs/gqlgen
	cp server/api/graphql/model/{models_gen.go,int64.go,bytes.go} ../core/api/model


.PHONY: tidy
tidy:
	go mod tidy


.PHONY: docker
docker:
	docker build -t $(DOCKER_IMAGE):latest .

.PHONY: docker_login
docker_login:
	docker login -u gitlab-ci-token -p ${CI_JOB_TOKEN} ${CI_REGISTRY}

.PHONY: docker_release
docker_release:
	docker push $(DOCKER_IMAGE):latest
