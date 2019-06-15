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
$ rustup default nightly-2019-05-10
```

3. Install [cargo-watch](https://github.com/passcod/cargo-watch)
```sh
$ cargo install --force cargo-watch
```

4. Install [diesel](http://diesel.rs/)
```sh
$ cargo install diesel_cli --no-default-features --features postgres
```

5. Launch a PostgreSQL database
```sh
$ docker run -d -e POSTGRES_USER=[USER_TO_CHANGE] -e POSTGRES_DB=[DB_TO_CHANGE] -e POSTGRES_PASSWORD=[PASSWORD_TO_CHANGE] -p 5432:5432 postgres:11
```

6. Edit `bloom.sane` with correct values
```sh
$ cd server
$ cp bloom.sane.template bloom.sane
# edit bloom.sane
$ cat bloom.sane
rust_env = "development"
host = "http://localhost:8080"
port = 8000
database = {
    url = "postgres://USER:PASSWORD@127.0.0.1:5432/DATABASE?sslmode=disable"
}
aws = {
    secret_access_key = "XXX",
    access_key_id = "XXX",
    region = "XXX",
}
s3 = {
    bucket = "XXX",
    base_url = "https://s3.REGION.amazonaws.com",
}
sentry = {
    url = "XXX" # optional
}
phaser = {
    secret = "XXX",
}
bitflow = {
    secret = "XXX",
}
smtp = {
    port = 587,
    host = "XXX",
    username = "XXX",
    password = "XXX",
}

blacklists = {
    email_domains = [
        "assets/disposable_email_domains.txt",
    ],
    passwords = [
        "assets/common_passwords_200.txt",
    ]
}
```

7. Run migrations
```sh
# still in server/
$ export DATABASE_URL=XXX # previously set in bloom.sane
$ diesel migration run
```

8. Run development server
```sh
# still in server/
$ make dev
```
