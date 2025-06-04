use crate::engine::*;
use macroquad::prelude::*;

pub struct Tile {
    pub glyph: Glyph,
    pub background: Option<Color>,
    pub transparent: bool,
    pub walkable: bool,
}

impl Tile {
    pub const FLOOR: Tile = Tile {
        glyph: Glyph {
            symbol: '.',
            color: LIGHTGRAY,
        },
        background: None,
        transparent: true,
        walkable: true,
    };

    pub const WALL: Tile = Tile {
        glyph: Glyph {
            symbol: '#',
            color: LIGHTGRAY,
        },
        background: Some(DARKGRAY),
        transparent: false,
        walkable: false,
    };
}
