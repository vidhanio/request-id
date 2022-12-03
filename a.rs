#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
pub struct MakeRequestIdCounter {
    #[allow(dead_code)]
    counter: Arc<AtomicUsize>,
}
#[automatically_derived]
impl ::core::clone::Clone for MakeRequestIdCounter {
    #[inline]
    fn clone(&self) -> MakeRequestIdCounter {
        MakeRequestIdCounter {
            counter: ::core::clone::Clone::clone(&self.counter),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for MakeRequestIdCounter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "MakeRequestIdCounter",
            "counter",
            &&self.counter,
        )
    }
}
#[automatically_derived]
impl ::core::default::Default for MakeRequestIdCounter {
    #[inline]
    fn default() -> MakeRequestIdCounter {
        MakeRequestIdCounter {
            counter: ::core::default::Default::default(),
        }
    }
}
impl MakeRequestIdCounter {
    pub fn new() -> Self {
        Default::default()
    }
}
