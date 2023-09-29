#![no_std]

extern crate alloc;

use core::time::Duration;

use playdate_sys::ffi::PlaydateAPI;

pub mod prelude;
mod input;
mod rt;
pub(crate) mod event;
pub(crate) mod mutex;
pub mod ty;
pub mod std;
pub mod macros;
pub use euclid as math;

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

#[allow(unused)]
pub trait Game {
    fn new() -> Self;
    fn start(&mut self, pd: Playdate) {}
    fn update(&mut self, pd: Playdate, dt: Duration) {}
    fn draw(&mut self, pd: Playdate) {}
}

#[doc(hidden)]
pub mod __private {
    use core::{mem, time::Duration};

    pub const unsafe fn uninit<T>() -> T {
        mem::MaybeUninit::uninit().assume_init()
    }

    use crate::api;
    pub use crate::mutex::Mutex;

    static T: Mutex<(bool, u64)> = Mutex::new((false, 0));

    /// DO NOT CALL! WILL BREAK YOUR GAME'S EVENT LOOP!
    pub fn dt() -> Duration {
        if !T.get().0 {
            T.get().1 = unsafe { (*api().system).getCurrentTimeMilliseconds.unwrap()().into() };
            T.get().0 = true;
        }
        Duration::from_millis(unsafe { (*api().system).getCurrentTimeMilliseconds.unwrap()().into() }) - Duration::from_millis(T.get().1)
    }
}