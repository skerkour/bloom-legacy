# Development instructions

TODO

<!-- ## Code organization

### Common

go packages shared across `server` and `core`

https://gitlab.com/bloom42/bloom/tree/master/common

### Server

The Bloom server

https://gitlab.com/bloom42/bloom/tree/master/server

### Core

A common piece of software shared across both `mobile` and `desktop`. Used to access the local DB and the API.

https://gitlab.com/bloom42/bloom/tree/master/core

### Mobile

The Bloom mobile applications (Android & iOS)

https://gitlab.com/bloom42/bloom/tree/master/mobile

### Desktop

The Bloom desktop applications (Linux & MacOS & Windows)

https://gitlab.com/bloom42/bloom/tree/master/desktop

## Bitflow

The Bitflow worker

https://gitlab.com/bloom42/bitflow



## Server

1. [Install Go](https://gitlab.com/bloom42/wiki/-/wikis/engineering/setup)
2. Clone [Bloom repository](https://gitlab.com/bloom42/bloom) and `cd` in the folder
```sh
git clone git@gitlab.com:bloom42/bloom.git && cd bloom/server
```
3. Launch a PostgreSQL instance
```sh
$ docker run -d -e POSTGRES_USER=[USER_TO_CHANGE] -e POSTGRES_PASSWORD=[PASSWORD_TO_CHANGE] -p 5432:5432 postgres:11
```
4. Copy `bloom.default.sane` to `bloom.sane`
```sh
$ cp bloom.default.sane bloom.sane
```
5. Edit `bloom.sane` with correct values
6. Run migrations
```sh
$ go run main.go migrations run
```
7. Run development server
```sh
$ make dev
```
## Core

1. [Install Go](https://gitlab.com/bloom42/wiki/-/wikis/engineering/setup)
2. `cd` to the folder
```sh
cd core
```
3. Build for your platform
```sh
make desktop # or make android_x86, or make ios...
```

## Desktop

1. [Install NodeJS and Go](/engineering/setup.html#nodejs)
2. Go to the `desktop` in the folder
```sh
cd desktop
```
3. Copy assets
```sh
$ make assets
```
4. Build core for desktop (see above)
5. run `make dev`
```sh
make dev
```

## Mobile

1. [Install Flutter and go](/engineering/setup.html#flutter)
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
5. Build core for mobile (see above)
6. Run the project in [Visual Studio Code](/engineering/setup.html#visual-studio-code)


## Resources

* https://medium.com/visly/rust-on-android-19f34a2fb43

 -->
