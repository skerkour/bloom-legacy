//! This crate aims to provide a convenient short-hand for returning early
//! from `futures`-based functions.
//!
//! The general pattern it supports is where before a function performs
//! an asynchronous task, it does some work that might result in an early
//! termination, for example:
//!
//! - certain parsing or validation logic that might fail, and upon which
//!   the function should return immediately with an error
//! - some local cache lookup or other optimization that might render the
//!   asynchronous task unnecessary, and where the function should instead
//!   return immediately with a value
//!
//! To that end, the [`TryFuture`](TryFuture) struct implements a future which can
//! either resolve immediately with a value or an error, or alternatively wrap another
//! future performing some asynchronous task.
//!
//! The equivalent of [`try!`](try!) or the `?` operator is provided by the
//! [`try_future!`](try_future!) and [`try_future_box!`](try_future_box!) macros:
//!
//! - `try_future!` will inspect the `Result` passed to it, and will exit
//! the current function by returning a `TryFuture` when it finds an `Err`. Otherwise
//! it will unwrap the result and pass back the value inside the result.
//!
//! - `try_future_box!` works in the same way, except that when encountering an `Err`
//! it will return a boxed future.
//!
//! > **Please note**: The `try_future!` and `try_future_box!` macros only accept inputs
//! > of type `Result`. Alas, the equivalent of `async`/`await` is not yet possible in
//! > stable rust (however if you don't shy away from using nightly, you could take a look
//! > at the [`futures-await`](https://github.com/alexcrichton/futures-await) project).
//!
//! # Examples
//!
//! ## Using `impl Future<_>`
//!
//! ```rust,ignore
//! #[macro_use] extern crate try_future;
//!
//! fn make_request<C: Connect>(target: &str, client: &Client<C>) ->
//!     impl Future<Item=Response, Error=Error>
//! {
//!     let uri = try_future!(target.parse::<Uri>());
//!
//!     client.get(uri).into()
//! }
//! ```
//!
//! ## Using `Box<Future<_>>`
//!
//! ```rust,ignore
//! #[macro_use] extern crate try_future;
//!
//! fn make_request<C: Connect>(target: &str, client: &Client<C>) ->
//!     Box<Future<Item=Response, Error=Error>>
//! {
//!     let uri = try_future_box!(target.parse::<Uri>());
//!
//!     Box::new(client.get(uri))
//! }
//! ```

extern crate futures;

use std::fmt::{Debug, Formatter, Result as FmtResult};

use futures::{Future, IntoFuture, Poll};

#[doc(hidden)]
pub use futures::future::err;

/// Future which can either resolve immediately with a value or an error,
/// or alternatively wrap another future performing some asynchronous task.
///
/// The [`From`](From) impl on `TryFuture` can be used to wrap another future.
pub struct TryFuture<F: Future> {
    inner: TryFutureInner<F>
}

impl<F: Future + Debug> Debug for TryFuture<F> {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        fmt.debug_struct("TryFuture").finish()
    }
}

#[derive(Debug)]
enum TryFutureInner<F: Future> {
   Result(::futures::future::FutureResult<F::Item, F::Error>),
   Future(F)
}

impl<F: Future> TryFuture<F> {
    fn from_result(res: Result<F::Item, F::Error>) -> TryFuture<F> {
        TryFuture {
            inner: TryFutureInner::Result(res.into_future())
        }
    }

    /// Create a `TryFuture` future that immediately resolves
    /// with the specified value.
    pub fn from_ok(item: F::Item) -> TryFuture<F> {
        TryFuture::from_result(Ok(item))
    }

    /// Create a `TryFuture` future that immediately rejects
    /// with the specified error.
    pub fn from_error(err: F::Error) -> TryFuture<F> {
        TryFuture::from_result(Err(err))
    }
}

impl<F: IntoFuture> From<F> for TryFuture<F::Future> {
    fn from(future: F) -> TryFuture<F::Future> {
        TryFuture {
            inner: TryFutureInner::Future(future.into_future())
        }
    }
}

impl<F: Future> Future for TryFuture<F> {
    type Item = F::Item;
    type Error = F::Error;

    fn poll(&mut self) -> Poll<F::Item, F::Error> {
        match self.inner {
            TryFutureInner::Result(ref mut future) => future.poll(),
            TryFutureInner::Future(ref mut future) => future.poll()
        }
    }
}

/// Equivalent of `try! returning a [`TryFuture`](TryFuture).
#[macro_export]
macro_rules! try_future {
    ($expression:expr) => (
        match $expression {
            Err(err) => {
                return $crate::TryFuture::from_error(err.into());
            },
            Ok(value) => value
        }
    )
}

/// Equivalent of `try!` returning a `Box<Future<_>>`.
#[macro_export]
macro_rules! try_future_box {
    ($expression:expr) => (
        match $expression {
            Err(err) => {
                return Box::new($crate::err(err.into()));
            },
            Ok(value) => value
        }
    )
}

#[cfg(test)]
mod tests {
    use futures::{future, Future};

    use super::TryFuture;

    #[test]
    fn test_from_ok() {
        let future = TryFuture::<future::Empty<u32, ()>>::from_ok(42);
        assert_eq!(42, future.wait().unwrap());
    }

    #[test]
    fn test_from_error() {
        let future = TryFuture::<future::Empty<u32, ()>>::from_error(());
        assert_eq!((), future.wait().err().unwrap());
    }

    #[test]
    fn test_ok_into() {
        let future: TryFuture<_> = future::ok::<u32, ()>(42).into();
        assert_eq!(42, future.wait().unwrap());
    }

    #[test]
    fn test_err_into() {
        let future: TryFuture<_> = future::err::<u32, ()>(()).into();
        assert_eq!((), future.wait().err().unwrap());
    }

    #[test]
    fn test_try_future() {
        use std::io;

        struct CustomError(io::Error);

        impl From<io::Error> for CustomError {
            fn from(err: io::Error) -> CustomError {
                CustomError(err)
            }
        }

        let future = future::lazy(|| {
            try_future!(Err(io::Error::new(io::ErrorKind::Other, "oh no!")));

            future::ok::<_, CustomError>(()).into()
        });

        assert_eq!("oh no!", future.wait().err().unwrap().0.to_string());
    }

    #[test]
    fn test_try_future_box() {
        use std::io;

        struct CustomError(io::Error);

        impl From<io::Error> for CustomError {
            fn from(err: io::Error) -> CustomError {
                CustomError(err)
            }
        }

        let future = future::lazy(|| {
            try_future_box!(Err(io::Error::new(io::ErrorKind::Other, "oh no!")));

            Box::new(future::ok::<_, CustomError>(()))
        });

        assert_eq!("oh no!", future.wait().err().unwrap().0.to_string());
    }
}
