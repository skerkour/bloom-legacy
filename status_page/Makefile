.PHONY: build docker_login release run

NAME := status_page
DOCKER_IMAGE = registry.gitlab.com/bloom42/$(NAME)

build:
	docker build -t $(DOCKER_IMAGE):latest .

docker_login:
	docker login -u gitlab-ci-token -p ${CI_JOB_TOKEN} ${CI_REGISTRY}

release:
	docker push $(DOCKER_IMAGE):latest

run:
	docker run -ti --rm -v ${PWD}/vigil.toml:/vigil/vigil.toml -p 8082:8080 $(DOCKER_IMAGE)
