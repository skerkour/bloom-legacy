NAME := bloomserver
# VERSION := $(shell cat Cargo.toml | grep '^version =' | cut -d '"' -f2)
COMMIT = $(shell git rev-parse HEAD)
DATE := $(shell date +"%Y-%m-%d")


.PHONY: all
all: build

.PHONY: fmt
fmt:
	go fmt ./...

.PHONY: test
test:
	go test ./...

.PHONY: build
build:
	go build -o dist/$(NAME)

.PHONY: clean
clean:
	rm -rf dist
