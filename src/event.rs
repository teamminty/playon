use alloc::{vec::Vec, boxed::Box};

use crate::{mutex::Mutex, ty::{Key, Button}};

pub static EVENTS: Mutex<Events> = Mutex::new(Events::new());

/// TODO
pub struct Events {
    pub(crate) keypress: (Vec<Key>, Vec<Box<dyn Fn(Key)>>),
    pub(crate) keyrelease: (Vec<Key>, Vec<Box<dyn Fn(Key)>>),
    pub(crate) buttonpress: (Vec<Button>, Vec<Box<dyn Fn(Button)>>)
}

impl Events {
    const fn new() -> Self {
        Self {
            keypress: (Vec::new(), Vec::new()),
            keyrelease: (Vec::new(), Vec::new()),
            buttonpress: (Vec::new(), Vec::new())
        }
    }
}