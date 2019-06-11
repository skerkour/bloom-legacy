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

4. Install `diesel`
```sh
$ cargo install diesel_cli --no-default-features --features postgres
```

5. Launch a PostgreSQL database
```sh
$ docker run -d -e POSTGRES_USER=bloom -e POSTGRES_DB=bloom -e POSTGRES_PASSWORD=PASSWORD -p 5432:5432 postgres:11
```

6. Edit `.env` with correct values
```sh
$ cd server
$ cp .env.example .env
# edit .env
$ cat .env
RUST_ENV=development
DATABASE_URL=postgres://bloom:PASSWORD_REDACTED@127.0.0.1:5432/bloom?sslmode=disable
HOST=http://localhost:8080
AWS_SECRET_ACCESS_KEY=XXX
AWS_ACCESS_KEY_ID=XXX
AWS_REGION=XXX
S3_BUCKET=XXX
S3_BASE_URL=https://s3.REGION.amazonaws.com
SENTRY_URL=XXX
PHASER_SECRET=XXX # Random passphrase
BITFLOW_SECRET=XXX # Random passphrase
SMTP_PORT=587
SMTP_HOST=XXX
SMTP_USERNAME=XXX
SMTP_PASSWORD=XXX
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
