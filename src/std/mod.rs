//! A small portion of the standary library ported to playdate

pub mod sync;

mod reexports {
    pub use alloc::borrow;
    pub use alloc::boxed;
    pub use alloc::collections;
    pub use alloc::ffi;
    pub use alloc::fmt;
    pub use alloc::format;
    pub use alloc::rc;
    pub use alloc::slice;
    pub use alloc::str;
    pub use alloc::string;
    pub use alloc::task;
    pub use alloc::vec;
}

pub use reexports::*;
pub use alloc::alloc;