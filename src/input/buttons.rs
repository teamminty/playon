use playdate_sys::ffi::PDButtons;

use crate::{api, ty::DPadState};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DPad {
    _private: core::marker::PhantomData<()>,
}

impl DPad {
    pub fn state(&self) -> DPadState {
        let (mut down, mut pushed, mut released) = (PDButtons(0), PDButtons(0), PDButtons(0));
        unsafe {
            (*api().system).getButtonState.unwrap()(
                &mut down as *mut _,
                &mut pushed as *mut _,
                &mut released as *mut _,
            )
        };
        DPadState {
            up: get_bit_at(down.0.into(), 0),
            down: get_bit_at(down.0.into(), 1),
            left: get_bit_at(down.0.into(), 2),
            right: get_bit_at(down.0.into(), 3),
        }
    }
    pub(crate) const fn new() -> Self {
        Self {
            _private: core::marker::PhantomData,
        }
    }
}

fn get_bit_at(input: u32, n: u8) -> bool {
    if n < 32 {
        input & (1 << n) != 0
    } else {
        false
    }
}
