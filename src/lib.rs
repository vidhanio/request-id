//! A few common `MakeRequestId` implementations for use with `tower-http`

#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![warn(missing_docs)]
#![allow(clippy::needless_doctest_main)]

use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use hyper::Request;
use tower_http::request_id::{MakeRequestId, RequestId};

#[cfg(feature = "uuid")]
use uuid::Uuid;

#[cfg(feature = "ulid")]
use ulid::Ulid;

/// A [`MakeRequestId`] that generates a [`RequestId`] from a [`Uuid`].
///
/// [`MakeRequestId`]: ::tower_http::request_id::MakeRequestId
/// [`RequestId`]: ::tower_http::request_id::RequestId
/// [`Uuid`]: ::uuid::Uuid
#[derive(Clone, Copy, Debug, Default)]
#[cfg(feature = "uuid")]
pub struct MakeRequestUuid;

#[cfg(feature = "uuid")]
impl MakeRequestId for MakeRequestUuid {
    fn make_request_id<B>(
        &mut self,
        #[allow(unused_variables)] request: &Request<B>,
    ) -> Option<RequestId> {
        Some(RequestId::new(
            Uuid::new_v4()
                .to_string()
                .parse()
                .expect("uuid should only contain ascii characters"),
        ))
    }
}

/// A [`MakeRequestId`] that generates a [`RequestId`] from a [`Ulid`].
///
/// [`MakeRequestId`]: ::tower_http::request_id::MakeRequestId
/// [`RequestId`]: ::tower_http::request_id::RequestId
/// [`Ulid`]: ::ulid::Ulid
#[derive(Clone, Copy, Debug, Default)]
#[cfg(feature = "ulid")]
pub struct MakeRequestUlid;

#[cfg(feature = "ulid")]
impl MakeRequestId for MakeRequestUlid {
    fn make_request_id<B>(
        &mut self,
        #[allow(unused_variables)] request: &Request<B>,
    ) -> Option<RequestId> {
        Some(RequestId::new(
            Ulid::new()
                .to_string()
                .parse()
                .expect("ulid should only contain ascii characters"),
        ))
    }
}

/// A [`MakeRequestId`] that uses an atomic counter to generate [`RequestId`]s.
///
/// [`MakeRequestId`]: ::tower_http::request_id::MakeRequestId
/// [`RequestId`]: ::tower_http::request_id::RequestId
#[derive(Clone, Debug, Default)]
pub struct MakeRequestIdCounter {
    counter: Arc<AtomicUsize>,
}

impl MakeRequestIdCounter {
    /// Create a new `MakeRequestIdCounter`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl MakeRequestId for MakeRequestIdCounter {
    fn make_request_id<B>(
        &mut self,
        #[allow(unused_variables)] request: &Request<B>,
    ) -> Option<RequestId> {
        Some(RequestId::new(
            self.counter
                .fetch_add(1, Ordering::Relaxed)
                .to_string()
                .parse()
                .expect("usize should only contain ascii characters"),
        ))
    }
}

#[cfg(test)]
mod tests {
    use hyper::Request;
    use tower_http::request_id::MakeRequestId;

    use super::*;

    #[test]
    fn make_request_id_counter() {
        let mut make_request_id = MakeRequestIdCounter::new();

        let next = make_request_id
            .make_request_id(&Request::new(()))
            .unwrap()
            .into_header_value();

        assert_eq!("0", next.to_str().unwrap());

        let next = make_request_id
            .make_request_id(&Request::new(()))
            .unwrap()
            .into_header_value();

        assert_eq!("1", next.to_str().unwrap());
    }

    #[test]
    #[cfg(feature = "uuid")]
    fn make_request_uuid() {
        let mut make_request_id = MakeRequestUuid;

        let next = make_request_id
            .make_request_id(&Request::new(()))
            .unwrap()
            .into_header_value();

        assert!(uuid::Uuid::try_parse_ascii(next.as_bytes()).is_ok());
    }

    #[test]
    #[cfg(feature = "ulid")]
    fn make_request_ulid() {
        let mut make_request_id = MakeRequestUlid;

        let next = make_request_id
            .make_request_id(&Request::new(()))
            .unwrap()
            .into_header_value();

        assert!(ulid::Ulid::from_string(next.to_str().unwrap()).is_ok());
    }
}
