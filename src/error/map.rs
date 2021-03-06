use http::status::StatusCode;
use util::BufStream;

use futures::{Future, Poll};

/// Map an arbitrary error type to [`Error`]
///
/// The conversion is done by treating all errors as "internal server errors".
///
/// [`Error`]: struct.Error.html
#[derive(Debug)]
pub struct Map<T> {
    inner: State<T>,
}

#[derive(Debug)]
enum State<T> {
    Inner(T),
    Immediate(Option<::Error>),
}

impl<T> Map<T> {
    /// Create a new `Map` instance backed by `inner`.
    ///
    /// The returned value will map all errors generated by `inner` into
    /// [`Error`] by treating them as "internal server errors".
    ///
    /// [`Error`]: struct.Error.html
    pub fn new(inner: T) -> Map<T> {
        Map {
            inner: State::Inner(inner),
        }
    }

    /// Create a neew `Map` instance that is in the error state.
    ///
    /// The instance will yield `error` immediately when it is used.
    pub fn immediate(error: ::Error) -> Map<T> {
        Map {
            inner: State::Immediate(Some(error)),
        }
    }
}

impl<T: Future> Future for Map<T> {
    type Item = T::Item;
    type Error = ::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use self::State::*;

        match self.inner {
            Inner(ref mut f) => f.poll().map_err(|_| ::Error::from(StatusCode::INTERNAL_SERVER_ERROR)),
            Immediate(ref mut e) => Err(e.take().unwrap()),
        }
    }
}

impl<T: BufStream> BufStream for Map<T> {
    type Item = T::Item;
    type Error = ::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        use self::State::*;

        match self.inner {
            Inner(ref mut f) => f.poll().map_err(|_| ::Error::from(StatusCode::INTERNAL_SERVER_ERROR)),
            Immediate(ref mut e) => Err(e.take().unwrap()),
        }
    }
}
