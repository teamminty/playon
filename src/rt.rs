use core::{ffi::{c_int, c_void}, sync::atomic::{AtomicBool, Ordering}};
use playdate_sys::ffi::{PDSystemEvent, PlaydateAPI};

use crate::ty::Key;

extern "Rust" {
    fn __playon_start();
    fn __playon_update() -> i32;
    fn __playon_postevent() -> i32;
}

static STARTED: AtomicBool = AtomicBool::new(false);

unsafe extern "C" fn on_update(_: *mut c_void) -> i32 {
    if STARTED.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
        __playon_start();
    }
    match __playon_update() {
        // 1 means success
        1 => {}
        o => return o
    };
    update_events();
    __playon_postevent()
}

#[no_mangle]
pub extern "C" fn eventHandlerShim(api: *const PlaydateAPI, event: PDSystemEvent, _arg: u32) -> c_int {
    match event {
		PDSystemEvent::kEventInit => unsafe {
			// register the API entry point
			playdate_sys::API = api;
			// get `setUpdateCallback` fn
			let f = (*(*api).system).setUpdateCallback.unwrap();
			// register update callback
			f(Some(on_update), core::ptr::null_mut());
		},

		PDSystemEvent::kEventLock => {/* TODO */},
		PDSystemEvent::kEventUnlock => {/* TODO */},
		PDSystemEvent::kEventPause => {/* TODO */},
		PDSystemEvent::kEventResume => {/* TODO */},
		PDSystemEvent::kEventLowPower => {/* TODO */},
		PDSystemEvent::kEventTerminate => {/* TODO */},
		PDSystemEvent::kEventInitLua => {/* TODO */},
		// simulator only, keyboard events:
		PDSystemEvent::kEventKeyPressed => {
            crate::event::EVENTS.get().keypress.0.push(Key);
        },
		PDSystemEvent::kEventKeyReleased => {
            crate::event::EVENTS.get().keyrelease.0.push(Key);
        },
	}
    0
}

fn update_events() {
    for b in &crate::event::EVENTS.get().keypress.0 {
        for v in &crate::event::EVENTS.get().keypress.1 {
            v(*b)
        }
    }
    let _ = &crate::event::EVENTS.get().keypress.0.drain(0..);
    for b in &crate::event::EVENTS.get().keyrelease.0 {
        for v in &crate::event::EVENTS.get().keyrelease.1 {
            v(*b)
        }
    }
    let _ = &crate::event::EVENTS.get().keyrelease.0.drain(0..);
    for b in &crate::event::EVENTS.get().buttonpress.0 {
        for v in &crate::event::EVENTS.get().buttonpress.1 {
            v(*b)
        }
    }
    let _ = &crate::event::EVENTS.get().buttonpress.0.drain(0..);
}