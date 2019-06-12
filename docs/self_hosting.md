# Self hosting


## Run Bloom

1. Install [Rust](https://rustup.rs/)

2. Install [diesel](http://diesel.rs/)
```sh
$ cargo install diesel_cli --no-default-features --features postgres
```

3. Launch a PostgreSQL database
```sh
$ docker run -d -e POSTGRES_USER=[USER_TO_CHANGE] -e POSTGRES_DB=[DB_TO_CHANGE] -e POSTGRES_PASSWORD=[PASSWORD_TO_CHANGE] -p 5432:5432 postgres:11
```

4. Clone Bloom's Git repository, go to the `server` directory, and copy `bloom.sane.template` to `bloom.sane`
```sh
$ git clone https://gitlab.com/bloom42/bloom.git
$ cd bloom/server
$ cp bloom.sane.template bloom.sane
```

5. Edit `bloom.sane` with correct values

6. Run migrations
```sh
$ export DATABASE_URL=XXX # previously set in bloom.sane
$ diesel migration run
```

7. Pull latest Docker image
```sh
$ docker pull registry.gitlab.com/bloom42/bloom:latest
```

8. Launch Docker container
```sh
$ docker run -d -p 8080:8000 -v `pwd`/bloom.sane:/bloom/bloom.sane:ro registry.gitlab.com/bloom42/bloom:latest
```
