# Development

Bloom use the following platforms to develop it's products:

* [NodeJS](#nodejs)
* [Rust](#rust)
* [Flutter](#flutter)


## NodeJS

Install NodeJS LTS from https://nodejs.org/en/download/

<!-- ```sh
$ cd webapp
$ echo 'VUE_APP_API_BASE_URL=http://0.0.0.0:8000/api' > .env.local
$ make install
$ make dev
``` -->

## Rust

1. Install [Rust](https://rustup.rs/)

2. Use our pinned Rust version
```sh
$ rustup default nightly-2019-07-19
# rustup component add clippy rls rust-analysis rust-src rustfmt
```

3. Install [cargo-watch](https://github.com/passcod/cargo-watch)
```sh
$ cargo install --force cargo-watch
```

4. Install [diesel](http://diesel.rs/)
```sh
$ cargo install diesel_cli --no-default-features --features postgres
```

<!-- 5. Launch a PostgreSQL database
```sh
$ docker run -d -e POSTGRES_USER=[USER_TO_CHANGE] -e POSTGRES_DB=[DB_TO_CHANGE] -e POSTGRES_PASSWORD=[PASSWORD_TO_CHANGE] -p 5432:5432 postgres:11
```

6. Edit `bloom.sane` with correct values
```sh
$ cd server
$ cp bloom.template.sane bloom.sane
# edit bloom.sane
$ cat bloom.sane
rust_env = "development"
host = "http://localhost:8080"
port = 8000

database = {
    url = "postgres://USER:PASSWORD@127.0.0.1:5432/DATABASE?sslmode=disable"
}

aws = {
    secret_access_key = "[XXX]",
    access_key_id = "[XXX]",
    region = "[XXX]",
}

s3 = {
    bucket = "[XXX]",
    base_url = "https://s3.REGION.amazonaws.com",
}

sentry = {
    server_url = "[XXX]" # optional
}

phaser = {
    # cryptographically secure random string of at least 65 characters
    secret = "[XXX]",
}

bitflow = {
    # cryptographically secure random string of at least 65 characters
    secret = "[XXX]",
}

smtp = {
    port = 587,
    # in development, host, username and password can be left as empty strings: ""
    # if empty, emails will be printed in the console
    host = "[XXX]",
    username = "[XXX]",
    password = "[XXX]",
}

stripe = {
    public_key = "[XXX]",
    secret_key = "[XXX]",
}

blacklists = {
    email_domains = [
        "assets/disposable_email_domains.txt",
    ],
    passwords = [
        "assets/common_passwords_200.txt",
    ]
}

disabled = [] # optional, you can disable service like disabled = ["phaser", "bitflow"]
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
``` -->


## Flutter

https://flutter.dev/docs/get-started/install
