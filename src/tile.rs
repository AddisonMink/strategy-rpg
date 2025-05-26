use crate::glyph::Glyph;
use macroquad::prelude::*;

pub struct Tile {
    pub glyph: Glyph,
    pub background: Option<Color>,
}

impl Tile {
    pub const FLOOR: Tile = Tile {
        glyph: Glyph {
            symbol: '.',
            color: LIGHTGRAY,
        },
        background: None,
    };

    pub const WALL: Tile = Tile {
        glyph: Glyph {
            symbol: '#',
            color: LIGHTGRAY,
        },
        background: Some(DARKGRAY),
    };
}
