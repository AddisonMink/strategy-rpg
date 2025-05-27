use crate::glyph::Glyph;
use macroquad::prelude::*;

pub struct Tile {
    pub glyph: Glyph,
    pub background: Option<Color>,
    pub transparent: bool,
}

impl Tile {
    pub const FLOOR: Tile = Tile {
        glyph: Glyph {
            symbol: '.',
            color: LIGHTGRAY,
        },
        background: None,
        transparent: true,
    };

    pub const WALL: Tile = Tile {
        glyph: Glyph {
            symbol: '#',
            color: LIGHTGRAY,
        },
        background: Some(DARKGRAY),
        transparent: false,
    };
}
