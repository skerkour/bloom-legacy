# Development


## Server

1. [Install Rust](/engineering/setup.html#rust)
2. Clone Bloom [server](https://gitlab.com/bloom42/server) and `cd` in the folder
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
2. Go to the `desktop` in the folder
```sh
cd desktop
```
3. in shell 1, go to the `native` directory, export env vars and run `make dev`
```sh
cd native
export npm_config_target=6.0.12
export npm_config_arch=x64
export npm_config_target_arch=x64
export npm_config_disturl=https://electronjs.org/headers
export npm_config_runtime=electron
export npm_config_build_from_source=true
make dev
```
4. In shell 2, install dependencies and run `make dev`
```sh
make install
make dev
```
5. In shell 3, run `npm start`
```sh
npm start
```


## Mobile

1. [Install Flutter](/engineering/setup.html#flutter)
2. go to the `mobile` folder
```sh
cd mobile
```
3. Install dependencies
```sh
make install
```
4. Run the project in [Visual Studio Code](/engineering/setup.html#visual-studio-code)
