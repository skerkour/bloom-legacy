# Caldyn, dynamic evaluation of mathematical expressions

[![Build Status](https://travis-ci.org/Luthaf/caldyn.svg?branch=master)](https://travis-ci.org/Luthaf/caldyn)
[![codecov](https://codecov.io/gh/Luthaf/caldyn/branch/master/graph/badge.svg)](https://codecov.io/gh/Luthaf/caldyn)
[![crates.io](https://img.shields.io/crates/v/caldyn.svg)](https://crates.io/crates/caldyn)

This crate provide run-time evaluation of mathematical expressions, embedded in
strings, containing constants and user-provided variables. This can be used to
evaluate user-provided expressions in a larger context.

## [Documentation](https://docs.rs/caldyn)

## Usage

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
caldyn = "0.4"
```

The easiest way to use this crate is with the `eval` function:

```rust
assert_eq!(caldyn::eval("3 + 5 * 2", None), Ok(13.0));
```

The second argument to `eval` is a `Context`, that can define variables:

```rust
use caldyn::Context;

let mut context = Context::new();
context.set("a", 3.5);
assert_eq!(caldyn::eval("2 * a", &context), Ok(7.0));
```

It is also possible to separate the parsing from the evaluation of an expression
with the `Expr` type. This allow to reuse the same expression with different
values for variables.

```rust
use caldyn::{Expr, Context};

let expr = Expr::parse("3 + 5 * 2").unwrap();
assert_eq!(expr.eval(None), Ok(13.0));

let expr = Expr::parse("3 / c + b").unwrap();
let mut context = Context::new();
context.set("c", 1.0);
context.set("b", 5.0);
assert_eq!(expr.eval(&context), Ok(8.0));

context.set("b", 10.0);
assert_eq!(expr.eval(&context), Ok(13.0));
```

It is also possible to set a callback function to be used when a variable is not
found in the context:

```rust
use caldyn::{eval, Context};

let mut context = Context::new();
context.set_query(|name| {
    match name {
        "a" | "b" | "c" => Some(1.0),
        _ => None
    }
});

assert_eq!(eval("a + b", &context), Ok(2.0));
// the following line would error with "undefined variable 'd'" message
// eval("d / 2", &context);
```

## Usage as a terminal calculator

You can use the [`calc`](examples/calc.rs) example as your terminal calculator,
installing it with cargo:

```
cargo install caldyn --example calc
```

## Language definition

The language implemented by caldyn can contain the following elements:

- float literal values: `-12.456`, `+0.0045e78`, ...;
- left and right parenthesis;
- mathematical operators: `+` for addition, `-` for subtraction,
  `*` for multiplication, `/` for division and `^` for exponentiation
  (`std::f64::powf`);
- variables. Variables names are ASCII only, and can start by a letter or
  `_`, and can contain letters, digits, `.`, `_`, `[` or `]`.
- function call: `sin(a)`, `atan(22.0)`. The following function are
  accessible, with the same meaning as the corresponding `std::f64`
  function: `sqrt`, `cbrt`, `sin`, `cos`, `tan`, `asin`, `acos`, `atan`,
  `sinh`, `cosh`, `tanh`, `asinh`, `acosh`, `atanh`, `floor`, `ceil`,
  `abs`, `exp`, `ln`, `log2`, `log10`.

Any other symbol is forbidden in the input.

The mathematical operators obey the usual relations of associativity and
precedence, but still carry the floating point properties: addition is not
commutative, `NaN` and infinities exist, ...

Please note that while `[` and `]` are allowed in variables names, nothing is
done with them. Users of caldyn can parse and interpret these as indexing
operators in their own [`Context::set_query()`] function.

## License and contributions

Caldyn is written by Guillaume Fraux and distributed under either the MIT or the
Apache license, at your choice. Contributions are welcome, please open an issue
before to discuss your changes !
