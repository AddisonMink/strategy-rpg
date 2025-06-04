use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Glyph {
    pub symbol: char,
    pub color: Color,
}

impl Glyph {
    pub const fn new(symbol: char, color: Color) -> Self {
        Glyph { symbol, color }
    }
}
