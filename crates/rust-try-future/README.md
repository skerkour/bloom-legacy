# `try_future`

[![Build Status](https://travis-ci.org/srijs/rust-try-future.svg?branch=master)](https://travis-ci.org/srijs/rust-try-future)
[![Dependency Status](https://deps.rs/repo/github/srijs/rust-try-future/status.svg)](https://deps.rs/repo/github/srijs/rust-try-future)
[![crates.io](https://img.shields.io/crates/v/try_future.svg)](https://crates.io/crates/try_future)

This crate aims to provide a convenient short-hand for returning early
from `futures`-based functions.

The general pattern it supports is where before a function performs
an asynchronous task, it does some work that might result in an early
termination, for example:

- certain parsing or validation logic might fail, upon which the function
  should return immediately with an error
- some local cache lookup or other optimization that might render the
  asynchronous task unnecessary, and where the function would want immediately
  return a value instead

# Examples

## Using `impl Future<_>`

```rust
#[macro_use] extern crate try_future;

fn make_request<C: Connect>(target: &str, client: &Client<C>) ->
    impl Future<Item=Response, Error=Error>
{
    let uri = try_future!(target.parse::<Uri>());

    client.get(uri).into()
}
```

## Using `Box<Future<_>>`

```rust
#[macro_use] extern crate try_future;

fn make_request<C: Connect>(target: &str, client: &Client<C>) ->
    Box<Future<Item=Response, Error=Error>>
{
    let uri = try_future_box!(target.parse::<Uri>());

    Box::new(client.get(uri))
}
```
