use alloc::{vec::Vec, boxed::Box};

use crate::{mutex::Mutex, ty::Key};

pub static EVENTS: Mutex<Events> = Mutex::new(Events::new());

/// TODO
pub struct Events {
    pub(crate) keypress: Vec<Box<dyn Fn(Key)>>,
    pub(crate) keyrelease: Vec<Box<dyn Fn(Key)>>
}

impl Events {
    const fn new() -> Self {
        Self {
            keypress: Vec::new(),
            keyrelease: Vec::new()
        }
    }
}