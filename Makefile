NAME = $(shell cat version/version.go| grep "\sName" | cut -d '"' -f2)
VERSION := $(shell cat version/version.go| grep "\sVersion\s=" | cut -d '"' -f2)
COMMIT = $(shell git rev-parse HEAD)
DATE := $(shell date +"%Y-%m-%d")
GO_PKG = $(shell cat go.mod | grep module | cut -d' ' -f2)

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
	# go run main.go server
	gowatch -build="make build" -exclude-dir=website -exclude-dir=mobile  -exclude-dir=desktop -command="dist/$(NAME) server run"

.PHONY: build
build:
	go build -o dist/$(NAME) -ldflags "-X $(GO_PKG)/version.GitCommit=$(COMMIT) \
		-X $(GO_PKG)/version.UTCBuildTime=`TZ=UTC date -u '+%Y-%m-%dT%H:%M:%SZ'` \
		-X $(GO_PKG)/version.GoVersion=`go version | cut -d' ' -f 3 | cut -c3-`"
	cp -r migrations dist/
	cp bloom.default.sane dist/bloom.sane


.PHONY: clean
clean:
	rm -rf dist


.PHONY: gqlgen
gqlgen:
	go run github.com/99designs/gqlgen -v
	cp server/api/graphql/model/{models_gen.go,int64.go,bytes.go} core/api/model


.PHONY: tidy
tidy:
	go mod tidy
