#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
pub use alloc::borrow;
#[cfg(feature = "alloc")]
pub use alloc::format;
#[cfg(feature = "alloc")]
pub use alloc::str;
#[cfg(feature = "alloc")]
pub use alloc::string;
#[cfg(feature = "alloc")]
pub use alloc::vec;
#[cfg(feature = "alloc")]
pub use core::convert;
#[cfg(feature = "alloc")]
pub use core::fmt;
#[cfg(feature = "alloc")]
pub use core::mem;
#[cfg(feature = "alloc")]
pub use core::ptr;

#[cfg(not(feature = "alloc"))]
pub use std::borrow;
#[cfg(not(feature = "alloc"))]
pub use std::convert;
#[cfg(not(feature = "alloc"))]
pub use std::fmt;
#[cfg(not(feature = "alloc"))]
pub use std::format;
#[cfg(not(feature = "alloc"))]
pub use std::mem;
#[cfg(not(feature = "alloc"))]
pub use std::ptr;
#[cfg(not(feature = "alloc"))]
pub use std::str;
#[cfg(not(feature = "alloc"))]
pub use std::string;
#[cfg(not(feature = "alloc"))]
pub use std::vec;

pub mod collections {
    #[cfg(feature = "alloc")]
    extern crate alloc;

    #[cfg(feature = "alloc")]
    pub use alloc::collections::BTreeMap;
    #[cfg(feature = "alloc")]
    pub use alloc::collections::BTreeSet;
    #[cfg(feature = "alloc")]
    pub use hashbrown::HashMap;
    #[cfg(feature = "alloc")]
    pub use hashbrown::HashSet;

    #[cfg(not(feature = "alloc"))]
    pub use std::collections::BTreeMap;
    #[cfg(not(feature = "alloc"))]
    pub use std::collections::BTreeSet;
    #[cfg(not(feature = "alloc"))]
    pub use std::collections::HashMap;
    #[cfg(not(feature = "alloc"))]
    pub use std::collections::HashSet;
}
