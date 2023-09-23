// TODO

pub struct Mutex<T> {
    v: crate::mutex::Mutex<T>
}

impl<T> Mutex<T> {
    pub const fn new(v: T) -> Self {
        Self {
            v: crate::mutex::Mutex::new(v)
        }
    }
}

pub struct MutexGuard<'a, T> {
    dat: &'a mut T
}