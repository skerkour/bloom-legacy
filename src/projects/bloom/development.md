# Development


## Server

1. [Install Rust](/engineering/setup.html#rust)
2. Clone Bloom [server](https://gitlab.com/bloom42/server) and go in it
```sh
git clone git@gitlab.com:bloom42/server.git && cd server
```
3. Launch a PostgreSQL instance
```sh
$ docker run -d -e POSTGRES_USER=[USER_TO_CHANGE] -e POSTGRES_DB=[DB_TO_CHANGE] -e POSTGRES_PASSWORD=[PASSWORD_TO_CHANGE] -p 5432:5432 postgres:11
```
4. Copy `bloom.default.sane` to `bloom.sane`
```sh
$ cp bloom.default.sane bloom.sane
```
5. Edit `bloom.sane` with correct values
6. Run migrations
```sh
$ export DATABASE_URL=XXX # previously set in bloom.sane
$ diesel migration run
```
8. Run development server
```sh
$ make dev
```


## Desktop

1. [Install NodeJS](/engineering/setup.html#nodejs)



## Mobile

1. [Install Flutter](/engineering/setup.html#flutter)


<!--
## Build


### Build locally

```sh
$ make
```

### Build Docker image

```sh
$ make docker_build
``` -->
