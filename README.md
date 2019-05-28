<p align="center">
  <img alt="bloom logo" src="https://bloom.sh/kernel/static/imgs/logos/bloom_256.png" height="180" />
  <h3 align="center">Bloom</h3>
</p>

--------

<!-- [![Build Status](https://travis-ci.com/z0mbie42/kernel.svg?token=8WFNhu6wffpdLjmEs8Fi&branch=master)](https://travis-ci.com/z0mbie42/kernel)
[![Build Status](https://dev.azure.com/bloom42/bloom/_apis/build/status/z0mbie42.kernel?branchName=master)](https://dev.azure.com/bloom42/bloom/_build/latest?definitionId=2&branchName=master) -->

1. [Documentation](#documentation)
2. [Docker image](#docker-image)
3. [Build](#build)
4. [Development](#Development)
5. [Test](#test)
6. [Contributing](#contributing)
7. [Licensing](#licensing)
8. [Sponsoring](#sponsoring)
9. [Security](#security)

--------

## Documentation

See the [Wiki](https://gitlab.com/bloom42/bloom/wikis)


## Docker image

[registry.gitlab.com/bloom42/bloom](https://gitlab.com/bloom42/bloom/container_registry)


## Build

```
$ make
```


## Development

### Server

```
$ cd server
$ make dev
```

### WebApp

```
$ cd webapp
$ make install
$ make dev
```


## Tests

### Server

```
$ cd server
$ make test
```

### WebApp

```
$ cd webapp
$ make test
```


## Contributing

Thank you for your interest in contributing! Please refer to
[https://bloom.sh/contribute](https://bloom.sh/contribute) for guidance.



## Licensing

See `LICENSE.txt` and [https://opensource.bloom.sh/licensing](https://opensource.bloom.sh/licensing)


## Sponsoring

Bloom is a free and open source project. If you are interested in supporting this project, the core team
and the contributors please consider:

* Becoming a patron:

<a href="https://www.patreon.com/bloom42" target="_blank" rel="noopener">
  <img src="https://c5.patreon.com/external/logo/become_a_patron_button.png" height="42"/>
</a>

* Donate on PayPal:

<a href="https://paypal.me/z0mbie42" target="_blank" rel="noopener">
  <img src="https://www.paypalobjects.com/en_US/FR/i/btn/btn_donateCC_LG.gif" height="42"/>
</a>


* Donate using crypto currencies:

**BTC**: `38QY24nHRkMxUFsEDobwJU2b5QzuSL39Yb`

**ETH**: `0x5121FE2A1014C4d57FCD2E8C4134A179851aFe6F`

**XMR**: `4GdoN7NCTi8a5gZug7PrwZNKjvHFmKeV11L6pNJPgj5QNEHsN6eeX3DaAQFwZ1ufD4LYCZKArktt113W7QjWvQ7CW7fRk3auob6QWFSgYJ`



If you want to have your logo on our [about page](https://bloom.sh/company/about) please visit our
[dedicated 'Become a sponsor' page](https://bloom.sh/become-a-sponsor).

✌️


## Security

If you found a security issue affecting this project, please do not open a public issue and refer to our
[dedicated security page](https://bloom.sh/company/security) instead. Tahnk you.
