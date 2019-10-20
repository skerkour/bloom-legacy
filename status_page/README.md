# Status page


## Usage

Edit configuration file with good values.

```bash
$ docker run -d --name status_page -v `pwd`/vigil.toml:/vigil/vigil.toml -p 8080:8080 registry.gitlab.com/bloom42/status_page:latest
```
