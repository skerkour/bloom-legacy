# sendgrid-rs
Unofficial Rust library for the SendGrid API.

[![Build Status](https://travis-ci.org/gsquire/sendgrid-rs.svg?branch=master)](https://travis-ci.org/gsquire/sendgrid-rs)

This crate requires Rust 1.15 or higher as it uses a crate that has a custom derive implementation.

sendgrid-rs implements all of the functionality of other supported SendGrid client libraries.
To use sendgrid-rs you must first create a SendGrid account and generate an API key. To create an API
key for your SendGrid account, use the account management interface or see the
[SendGrid API Documentation](https://sendgrid.com/docs/API_Reference/Web_API_v3/API_Keys/index.html).

sendgrid-rs is available on [crates.io](https://crates.io/crates/sendgrid) and can be included in your Cargo.toml as follows:

```toml
[dependencies]
sendgrid = "X.X.X"
```

## Build Dependencies
This library utilises [reqwest](https://crates.io/crates/reqwest). Follow the instructions on the
[reqwest README](https://github.com/seanmonstar/reqwest#requirements) in order to enable sending HTTPS
requests to the SendGrid API.

## Example
An example of using this library can be found in the examples directory. This example code expects to
find your SendGrid API key in the process environment. In shells such as Bash or ZSH this can be set as follows:

```shell
export SENDGRID_API_KEY="SG.my.api.key"
```

## Documentation
[Documentation](https://docs.rs/sendgrid)

Please don't hesitate to contact me at the email listed in my profile. I will
try to help as quickly as I can. If you would like to contribute, contact me
as well.

## Mentions
Thanks to [meehow](https://github.com/meehow) for their contributions.

Thanks to [richo](https://github.com/richo) for their improvements to the V2 API.

## License
MIT
