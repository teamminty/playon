use crate::api;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Crank {
    _private: core::marker::PhantomData<()>
}

impl Crank {
    /// Returns `true` if the crank is docked.
    pub fn is_docked(&self) -> bool {
        unsafe {(*api().system).isCrankDocked.unwrap()() != 0}
    }
    /// Returns the current angle of the crank.
    pub fn rotation(&self) -> euclid::Angle<f32> {
        unsafe {euclid::Angle::degrees((*api().system).getCrankAngle.unwrap()())}
    }
    pub fn sounds_disabled(&self) -> bool {
        let old = unsafe {(*api().system).setCrankSoundsDisabled.unwrap()(0)};
        unsafe {(*api().system).setCrankSoundsDisabled.unwrap()(old)};
        old != 0
    }
    pub fn set_sounds_disabled(&self, v: bool) -> bool {
        unsafe {(*api().system).setCrankSoundsDisabled.unwrap()(v as i32) != 0}
    }
    pub(crate) const fn new() -> Self {
        Self {
            _private: core::marker::PhantomData
        }
    }
}