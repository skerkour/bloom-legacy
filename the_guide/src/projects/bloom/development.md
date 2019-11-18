# Development


## Server

1. [Install Rust](/engineering/setup.html#rust)
2. Clone Bloom [server](https://gitlab.com/bloom42/server) and `cd` in the folder
```sh
git clone git@gitlab.com:bloom42/server.git && cd server
```
3. Launch a PostgreSQL instance
```sh
$ docker run -d -e POSTGRES_USER=[USER_TO_CHANGE] -e POSTGRES_DB=[DB_TO_CHANGE] -e POSTGRES_PASSWORD=[PASSWORD_TO_CHANGE] -p 5432:5432 postgres:12
```
4. Copy `bloom.default.sane` to `bloom.sane`
```sh
$ cp bloom.default.sane bloom.sane
```
5. Edit `bloom.sane` with correct values
6. Copy assets
```sh
$ make assets
```
7. Run migrations
```sh
$ cargo run -- migrations run
```
8. Run development server
```sh
$ make dev
```


## Desktop

1. [Install NodeJS](/engineering/setup.html#nodejs)
2. Go to the `desktop` in the folder
```sh
cd desktop
```
3. Copy assets
```sh
$ make assets
```
4. in shell 1, run `make dev_native`
```sh
make dev_native
```
5. In shell 2, install dependencies and run `make dev_electron`
```sh
make install
make dev_electron
```


## Mobile

1. [Install Flutter](/engineering/setup.html#flutter)
2. go to the `mobile` folder
```sh
cd mobile
```
3. Copy assets
```sh
$ make assets
```
4. Install dependencies
```sh
make install
```
5. Run the project in [Visual Studio Code](/engineering/setup.html#visual-studio-code)


## Resources

* https://medium.com/visly/rust-on-android-19f34a2fb43
