.PHONY: build clean re dev test release lint
.PHONY: docker


NAME = $(shell cat version/version.go| grep "\sName" | cut -d '"' -f2)
DIST_DIR = dist
VERSION := $(shell cat version/version.go| grep "\sVersion\s=" | cut -d '"' -f2)
DOCKER_IMAGE = "registry.gitlab.com/bloom42/{PROJECT}"
COMMIT = $(shell git rev-parse HEAD)

build:
	cp -r assets $(DIST_DIR)
	go build -o $(DIST_DIR)/$(NAME)

dev:
	go run main.go

clean:
	rm -rf $(DIST_DIR)

re: clean build

lint:
	revive -formatter stylish ./...

test:
	go vet ./...
	go test -v -race ./...

release: clean
	git tag v$(VERSION)
	git push origin v$(VERSION)

static:
	CGO_ENABLED=0 go build -a -installsuffix cgo -ldflags="-w -s" -o $(DIST_DIR)/$(NAME)

gql: gql_account gql_drive gql_bitflow gql_phaser

gql_account:
	go run scripts/gqlgen/main.go -c apps/account/{PROJECT}/gqlgen.yml

gql_drive:
	go run scripts/gqlgen/main.go -c apps/drive/{PROJECT}/gqlgen.yml

gql_bitflow:
	go run scripts/gqlgen/main.go -c apps/bitflow/{PROJECT}/gqlgen.yml

gql_phaser:
	go run scripts/gqlgen/main.go -c apps/platform/phaser/{PROJECT}/gqlgen.yml


docker:
	docker build -t $(DOCKER_IMAGE):latest

docker_push:
	docker push $(DOCKER_IMAGE):latest

docker_release: docker
	docker tag $(DOCKER_IMAGE):latest $(DOCKER_IMAGE):$(VERSION)
	docker push $(DOCKER_IMAGE):$(VERSION)
