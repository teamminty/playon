use crate::consts::PIXEL_SIZE_MM;
pub use crate::engine::ty::*;

/// TODO
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Key;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Button {
    A,
    B
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DPadState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

trait Unit {
    fn to_pixels(v: f32) -> f32;
}

pub struct Millimeter;
impl Unit for Millimeter {
    fn to_pixels(v: f32) -> f32 {
        v / PIXEL_SIZE_MM
    }
}

pub struct Pixel;
impl Unit for Pixel {
    fn to_pixels(v: f32) -> f32 {
        v
    }
}