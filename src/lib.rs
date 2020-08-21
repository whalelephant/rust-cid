//! # cid
//!
//! Implementation of [cid](https://github.com/ipld/cid) in Rust.
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]

extern crate alloc;

mod cid;
mod codec;
mod error;
mod version;

#[cfg(any(test, feature = "test"))]
mod arb;

pub use self::cid::{Cid, CidGeneric};
pub use self::codec::Codec;
pub use self::error::{Error, Result};
pub use self::version::Version;
