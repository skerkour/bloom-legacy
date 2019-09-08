# CircleCI


## Disclaimer: not good
```bash
$ cat .circleci/config.yml
```
```yml
version: 2 # use CircleCI 2.0
jobs: # basic units of work in a run
  test: # runs not using Workflows must have a `build` job as entry point
    docker: # run the steps with Docker
      - image: golang:1.11
    working_directory: ~/workspace

    environment: # environment variables for the build itself
      GO111MODULE: 'on'

    steps: # steps that comprise the `build` job
      - checkout
      - run: make test

  build: # runs not using Workflows must have a `build` job as entry point
    machine:
      docker_layer_caching: true

    steps: # steps that comprise the `build` job
      - checkout
      - run: make docker

  release: # runs not using Workflows must have a `build` job as entry point
    machine:
      docker_layer_caching: true

    steps: # steps that comprise the `build` job
      - checkout
      - run: echo $DOCKER_PASSWORD | docker login -u $DOCKER_USERNAME --password-stdin $DOCKER_REGISTRY
      - run: make docker
      - run: make docker_release


workflows:
  version: 2
  test-build-release:
    jobs:
      - test:
          filters:  # required since `deploy` has tag filters AND requires `build`
            tags:
              only: /.*/
      - build:
          requires:
            - test
          filters:  # required since `deploy` has tag filters AND requires `build`
            tags:
              only: /.*/
      - release:
          requires:
            - build
          filters: # runs for no branches and only for tags starting with ‘v’.
            tags:
              only: /^v.*/
            branches:
              ignore: /.*/
```
