//! Caldyn, a crate for dynamic evaluation of mathematical expressions.
//!
//! This crate provide run-time evaluation of mathematical expressions,
//! embedded in strings. The easiest way to use this crate is with the
//! [`eval`](fn.eval.html) function:
//!
//! ```
//! assert_eq!(caldyn::eval("3 + 5 * 2", None), Ok(13.0));
//! ```
//!
//! The second argument to `eval` is a [`Context`](struct.Context.html), that
//! can define variables:
//!
//! ```
//! use caldyn::Context;
//!
//! let mut context = Context::new();
//! context.set("a", 3.5);
//! assert_eq!(caldyn::eval("2 * a", &context), Ok(7.0));
//! ```
//!
//! It is also possible to separate the parsing from the evaluation of an
//! expression with the [`Expr`](struct.Expr.html) type. This allow to reuse
//! the same expression with different values for variables.
//!
//! ```
//! use caldyn::{Expr, Context};
//!
//! let expr = Expr::parse("3 + 5 * 2").unwrap();
//! assert_eq!(expr.eval(None), Ok(13.0));
//!
//! let expr = Expr::parse("3 / c + b").unwrap();
//! let mut context = Context::new();
//! context.set("c", 1.0);
//! context.set("b", 5.0);
//! assert_eq!(expr.eval(&context), Ok(8.0));
//!
//! context.set("b", 10.0);
//! assert_eq!(expr.eval(&context), Ok(13.0));
//! ```
//!
//! It is also possible to set a callback function to be used when a variable
//! is not found in the context:
//!
//! ```
//! use caldyn::{eval, Context};
//!
//! let mut context = Context::new();
//! context.set_query(|name| {
//!     match name {
//!         "a" | "b" | "c" => Some(1.0),
//!         _ => None
//!     }
//! });
//!
//! assert_eq!(eval("a + b", &context), Ok(2.0));
//! // the following line would error with "undefined variable 'd'" message
//! // eval("d / 2", &context);
//! ```
//!
//! # Language definition
//!
//! The language implemented by caldyn can contain the following elements:
//!
//! - float literal values: `-12.456`, `+0.0045e78`, ...;
//! - left and right parenthesis;
//! - mathematical operators: `+` for addition, `-` for subtraction,
//!   `*` for multiplication, `/` for division and `^` for exponentiation
//!   (`std::f64::powf`);
//! - variables. Variables names are ASCII only, and can start by a letter or
//!   `_`, and can contain letters, digits, `.`, `_`, `[` or `]`.
//! - function call: `sin(a)`, `atan(22.0)`. The following function are
//!   accessible, with the same meaning as the corresponding `std::f64`
//!   function: `sqrt`, `cbrt`, `sin`, `cos`, `tan`, `asin`, `acos`, `atan`,
//!   `sinh`, `cosh`, `tanh`, `asinh`, `acosh`, `atanh`, `floor`, `ceil`,
//!   `abs`, `exp`, `ln`, `log2`, `log10`.
//!
//! Any other symbol is forbidden in the input.
//!
//! The mathematical operators obey the usual relations of associativity and
//! precedence, but still carry the floating point properties: addition is not
//! commutative, `NaN` and infinities exist, ...
//!
//! Please note that while `[` and `]` are allowed in variables names, nothing
//! is done with them. Users of caldyn can parse and interpret these as
//! indexing operators in their own [`Context::set_query()`] function.
//!
//! [`Context::set_query()`]: struct.Context.html#method.set_query
//!
//! # Technical details
//!
//! caldyn is based on an AST interpreter, and uses a simple Shuntting-Yard
//! algorithm for parsing the expressions. It works only with `f64` data, and
//! perform a simple constant propagation to optimize the expressions.

#[allow(clippy::all)]
mod context;
mod error;
mod expr;

pub use crate::context::Context;
pub use crate::error::Error;
pub use crate::expr::{eval, is_variable, Expr};
