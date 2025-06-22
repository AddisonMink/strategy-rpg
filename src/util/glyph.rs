use crate::engine_v2::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Glyph {
    pub symbol: char,
    pub color: Color,
}

impl Glyph {
    pub const fn new(symbol: char, color: Color) -> Self {
        Glyph { symbol, color }
    }

    pub fn with_alpha(self, alpha: f32) -> Self {
        let new_color = self.color.with_alpha(alpha);
        Glyph::new(self.symbol, new_color)
    }

    pub fn mix_color(&self, other: Color, factor: f32) -> Self {
        let mixed_color = mix_color(self.color, other, factor);
        Glyph::new(self.symbol, mixed_color)
    }
}
