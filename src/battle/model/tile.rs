use crate::engine::*;

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub name: ShortString,
    pub glyph: Glyph,
    pub bg_color: Option<Color>,
    pub walkable: bool,
}

impl Tile {
    pub const FLOOR: Self = Self {
        name: ShortString::new("Floor"),
        glyph: Glyph::new('.', LIGHTGRAY),
        bg_color: None,
        walkable: true,
    };

    pub const WALL: Self = Self {
        name: ShortString::new("Wall"),
        glyph: Glyph::new('#', GRAY),
        bg_color: Some(DARKGRAY),
        walkable: false,
    };
}
