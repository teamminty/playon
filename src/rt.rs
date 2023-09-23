use core::{ffi::{c_int, c_void}, sync::atomic::{AtomicBool, Ordering}};
use playdate_sys::ffi::{PDSystemEvent, PlaydateAPI};

extern "Rust" {
    fn __playon_start();
    fn __playon_update() -> i32;
}

static STARTED: AtomicBool = AtomicBool::new(false);

unsafe extern "C" fn on_update(_: *mut c_void) -> i32 {
    if STARTED.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
        __playon_start();
    }
    __playon_update()
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
		PDSystemEvent::kEventKeyPressed => {/* TODO */},
		PDSystemEvent::kEventKeyReleased => {/* TODO */},
	}
    0
}