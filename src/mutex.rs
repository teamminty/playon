use core::cell::UnsafeCell;

#[repr(transparent)]
pub struct Mutex<T> {
    data: UnsafeCell<T>
}

impl<T> Mutex<T> {
    pub fn get(&self) -> &mut T {
        // SAFETY: Its an unsafecell dumb*ss
        unsafe { &mut *self.data.get() }
    }
    pub const fn new(v: T) -> Self {
        Self { data: UnsafeCell::new(v)  }
    }
}

// SAFETY: Playdate doesn't have threading. PLEASE CONTACT ME IF THIS IS WRONG AND I COULD BE CRASHING PROGRAMS!
unsafe impl<T> Sync for Mutex<T> {}

impl<T: Clone> Clone for Mutex<T> where UnsafeCell<T>: Clone {
    fn clone(&self) -> Self {
        Self { data: self.data.clone() }
    }
}
impl<T: Copy> Copy for Mutex<T> where UnsafeCell<T>: Copy {}