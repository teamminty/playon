//! Playon engine

use ty::{Color, TextStyle, Vec2};
use crate::std::string::String;
pub(crate) mod ty;
pub(crate) mod prelude;
pub mod drawable;

pub trait RenderTarget {
    /// Clears the screen. Don't know why you would want this.
    fn clear<C: Color>(&mut self, color: C, loc: Vec2);
    /// Draw some text.
    fn draw_text(&mut self, text: String, style: TextStyle, loc: Vec2);
}

pub trait Render {
    fn render<T: RenderTarget>(&self, target: T);
}