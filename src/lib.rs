#![no_std]

extern crate alloc;

use playdate_sys::ffi::PlaydateAPI;

pub mod prelude;
mod input;
mod rt;
pub(crate) mod event;
pub(crate) mod mutex;
pub mod ty;
pub mod std;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Playdate {
    _private: core::marker::PhantomData<()>
}

impl Playdate {
    /// Returns the current playdate device.
    pub fn current() -> Self {
        Self {
            _private: core::marker::PhantomData
        }
    }
}

pub(crate) fn api() -> &'static PlaydateAPI {
    playdate_sys::api!()
}