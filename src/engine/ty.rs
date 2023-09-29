use core::ops::{Add, Sub};

use playdate_sys::ffi::LCDSolidColor;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum SolidColor {
    Black,
    White,
    Clear,
    XOR,
}

impl From<LCDSolidColor> for SolidColor {
    fn from(value: LCDSolidColor) -> Self {
        match value {
            LCDSolidColor::kColorBlack => Self::Black,
            LCDSolidColor::kColorWhite => Self::White,
            LCDSolidColor::kColorClear => Self::Clear,
            LCDSolidColor::kColorXOR => Self::XOR,
        }
    }
}

pub trait Color {
    fn pattern(&self) -> [SolidColor; 16];
}

pub enum Colors {
    Black,
    DarkerGrey,
    DarkGrey,
    Grey,
    LightGrey,
    LighterGrey,
    White,
}

impl Color for Colors {
    fn pattern(&self) -> [SolidColor; 16] {
        use SolidColor::*;
        match self {
            Self::Black => [Black; 16],
            Self::LighterGrey => [
                Black, White, White, White,
                White, White, White, White,
                White, White, White, White,
                White, White, White, White,
                // Black, White, White, White,
                // White, White, White, White,
                // White, White, White, White,
                // White, White, White, White,
            ],
            Self::LightGrey => [
                Black, White, White, White,
                White, White, White, White,
                White, White, Black, White,
                White, White, White, White,
                // Black, White, White, White,
                // White, White, White, White,
                // White, White, Black, White,
                // White, White, White, White,
            ],
            Self::Grey => [
                Black, White, Black, White,
                White, Black, White, Black,
                Black, White, Black, White,
                White, Black, White, Black,
                // Black, White, Black, White,
                // White, Black, White, Black,
                // Black, White, Black, White,
                // White, Black, White, Black,
            ],
            Self::DarkGrey => [
                White, Black, Black, Black,
                Black, Black, Black, Black,
                Black, Black, White, Black,
                Black, Black, Black, Black,
                // White, Black, Black, Black,
                // Black, Black, Black, Black,
                // Black, Black, White, Black,
                // Black, Black, Black, Black,
            ],
            Self::DarkerGrey => [
                White, Black, Black, Black,
                Black, Black, Black, Black,
                Black, Black, Black, Black,
                Black, Black, Black, Black,                
                // White, Black, Black, Black,
                // Black, Black, Black, Black,
                // Black, Black, Black, Black,
                // Black, Black, Black, Black,
            ],
            Self::White => [White; 16],
        }
    }
}

#[derive(Clone, Copy)]
pub struct TextStyle {
    pub color: SolidColor,
    pub leading: euclid::Length<f32, crate::ty::Pixel>,
    pub tracking: euclid::Length<f32, crate::ty::Pixel>
}

#[derive(Clone, Copy)]
pub struct Vec2 {
    x: euclid::Length<f32, crate::ty::Pixel>,
    y: euclid::Length<f32, crate::ty::Pixel>
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}