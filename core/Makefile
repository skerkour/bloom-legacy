NAME := core
# VERSION := $(shell cat Cargo.toml | grep '^version =' | cut -d '"' -f2)
COMMIT = $(shell git rev-parse HEAD)
DATE := $(shell date +"%Y-%m-%d")


.PHONY: all
all: build

.PHONY: build
build:
	go build -o dist/lib$(NAME).so -buildmode=c-shared ffi/ffi.go

PHONY: build_android
build_android:
	GOOS=android GOARCH=arm64 CGO_ENABLED=1 CC=aarch64-linux-android28-clang go build --tags "android" -o dist/lib$(NAME).so -buildmode=c-shared ffi/ffi.go
