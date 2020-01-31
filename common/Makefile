COMMIT = $(shell git rev-parse HEAD)
DATE := $(shell date +"%Y-%m-%d")

.PHONY: all
all: test

.PHONY: fmt
fmt:
	go fmt ./...

.PHONY: test
test:
	go test ./...
