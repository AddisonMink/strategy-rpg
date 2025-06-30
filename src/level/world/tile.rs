use crate::engine::*;
use crate::util::*;

#[derive(Debug, Clone)]
pub struct Tile {
    pub name: ShortString,
    pub glyph: Glyph,
    pub bg_color: Option<Color>,
    pub walkable: bool,
    pub transparent: bool,
    pub goal: bool,
}

impl Tile {
    pub const FLOOR: Self = Self {
        name: ShortString::new("Floor"),
        glyph: Glyph::new('.', LIGHTGRAY),
        bg_color: None,
        walkable: true,
        transparent: true,
        goal: false,
    };

    pub const WALL: Self = Self {
        name: ShortString::new("Wall"),
        glyph: Glyph::new('#', GRAY),
        bg_color: Some(DARKGRAY),
        walkable: false,
        transparent: false,
        goal: false,
    };

    pub const GOAL: Self = Self {
        name: ShortString::new("Goal"),
        glyph: Glyph::new('<', WHITE),
        bg_color: None,
        walkable: true,
        transparent: true,
        goal: true,
    };
}
