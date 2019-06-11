# Development

## WebApp

```sh
$ cd webapp
$ make install
$ make dev
```

## Server

1. Install [Rust](https://rustup.rs/)

2. Use our pinned Rust version
```sh
$ rustup default nightly-2019-05-09
```

3. Install `cargo-watch`
```sh
$ cargo install --force cargo-watch
```

4. Launch a PostgreSQL database
```sh
$ docker run -d -e POSTGRES_USER=bloom -e POSTGRES_DB=bloom -e POSTGRES_PASSWORD=PASSWORD postgres:11
```

6. Edit `.env` with correct values
```sh
$ cd server
$ cp .env.example .env
# edit .env
```

7. Run migrations
```sh
# still in server
$ diesel migration run
```

8. Run server
```sh
$ make dev
```
