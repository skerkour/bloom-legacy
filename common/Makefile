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

.PHONY: rpc
rpc:
	protoc --proto_path=${GOPATH}/src:. --twirp_out=. --go_out=. ./rpc/accounts/service.proto
	protoc --proto_path=${GOPATH}/src:. --twirp_out=. --go_out=. ./rpc/billing/service.proto
	protoc --proto_path=${GOPATH}/src:. --twirp_out=. --go_out=. ./rpc/groups/service.proto
