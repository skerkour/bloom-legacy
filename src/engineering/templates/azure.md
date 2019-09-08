# Azure


## .azure-pipelines.yml

```yml
trigger:
- master

pool:
  vmImage: 'Ubuntu-16.04'

variables:
  - group: phaser

steps:
  - script: |
      curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable
      echo "##vso[task.setvariable variable=PATH;]$PATH:$HOME/.cargo/bin"
    displayName: 'Install rust'
  - script: |
      sudo apt-get install -y ca-certificates libssl-dev pkg-config
    displayName: 'Install dependencies'
  - script: make test
    displayName: 'test'

  - script: make docker_build
    displayName: 'docker build'
```
