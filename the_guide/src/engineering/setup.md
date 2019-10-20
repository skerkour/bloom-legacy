# Development

Bloom use the following platforms to develop it's products:

* [Rust](#rust)
* [NodeJS](#nodejs)
* [Flutter](#flutter)

## Rust

1. Install [Rustup](https://rustup.rs/)

2. Use our pinned Rust version
```sh
$ rustup default nightly-2019-10-13
$ rustup component add clippy rls rust-analysis rust-src rustfmt
$ rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android
```

3. Install [cargo-watch](https://github.com/passcod/cargo-watch)
```sh
$ cargo install --force cargo-watch
```

4. Install [diesel](http://diesel.rs/)
```sh
$ cargo install diesel_cli --no-default-features --features postgres
```

## NodeJS

Install NodeJS LTS from https://nodejs.org/en/download/

<!-- ```sh
$ cd webapp
$ echo 'VUE_APP_API_BASE_URL=http://0.0.0.0:8000/api' > .env.local
$ make install
$ make dev
``` -->

## Flutter

https://flutter.dev/docs/get-started/install


## Visual Studio Code

https://code.visualstudio.com
