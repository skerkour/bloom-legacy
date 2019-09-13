# Travis


## Rust

```yml
sudo: required
language: rust
services:
  - docker

rust:
  - 1.33.0

# Need to cache the whole .cargo directory to keep .crates.toml for
# cargo-update to work
cache:
  directories:
    - /home/travis/.cargo

# But don't cache the cargo registry
before_cache:
  - rm -rf /home/travis/.cargo/registry


before_install:
  - rustup self update

script:
  - make test
  # - make lint
  # - make build
  - make docker_build

deploy:
  - provider: script
    skip_cleanup: true # Important, otherwise the build output would be purged.
    script: make crates_login && make crates_publish
    on:
      tags: true
  - provider: releases
    api_key: $GITHUB_TOKEN
    on:
      tags: true
  - provider: script
    script: make docker_login && make docker_release
    on:
      tags: true

notifications:
  email:
    on_success: never
```
