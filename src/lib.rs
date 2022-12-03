use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

macro_rules! make_request_id {
    {
        $([$feature:literal])?
        $Type:ident;
        |$($this:ident)?| $create:expr
     } => {
        $(#[cfg(feature = $feature)])*
        impl ::tower_http::request_id::MakeRequestId for $Type {
            fn make_request_id<B>(
                &mut self,
                #[allow(unused_variables)] request: &::hyper::Request<B>,
            ) -> ::std::option::Option<::tower_http::request_id::RequestId> {
                $(let $this = self;)*

                ::std::option::Option::Some(::tower_http::request_id::RequestId::new(
                    ::std::str::FromStr::from_str(&::std::string::ToString::to_string(&$create))
                        .expect("request id should only contain ascii characters"),
                ))
            }
        }
    };
}

#[derive(Clone, Copy, Debug, Default)]
#[cfg(feature = "uuid")]
pub struct MakeRequestUuid;

make_request_id! {
    ["uuid"]
    MakeRequestUuid;
    | | uuid::Uuid::new_v4()
}

#[derive(Clone, Copy, Debug, Default)]
#[cfg(feature = "ulid")]
pub struct MakeRequestUlid;

make_request_id! {
    ["ulid"]
    MakeRequestUlid;
    | | ulid::Ulid::new()
}

#[derive(Clone, Debug, Default)]
pub struct MakeRequestIdCounter {
    counter: Arc<AtomicUsize>,
}

impl MakeRequestIdCounter {
    pub fn new() -> Self {
        Default::default()
    }
}

make_request_id!(
    MakeRequestIdCounter;
    |this| this.counter.fetch_add(1, Ordering::SeqCst)
);

#[cfg(test)]
mod tests {
    use hyper::{http::HeaderValue, Request};
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
