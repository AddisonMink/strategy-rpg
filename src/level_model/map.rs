use crate::engine::*;

#[derive(Debug, Clone, Copy)]
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

pub struct Map {
    tiles: Vec<Tile>,
}

impl Map {
    pub const WIDTH: u16 = grid::WIDTH;
    pub const HEIGHT: u16 = grid::HEIGHT;

    pub const FLOOR_PLAN: [&str; 10] = [
        "##########",
        "#........#",
        "#..####..#",
        "#..#..#..#",
        "#..#..#..<",
        "#..####..#",
        "#........#",
        "#..####..#",
        "#........#",
        "##########",
    ];

    pub fn new() -> Self {
        let mut tiles = Vec::with_capacity((Self::WIDTH * Self::HEIGHT) as usize);

        for row in Self::FLOOR_PLAN.iter() {
            for ch in row.chars() {
                let tile = match ch {
                    '#' => Tile::WALL,
                    '.' => Tile::FLOOR,
                    '<' => Tile::GOAL,
                    _ => Tile::FLOOR, // fallback for unknown chars
                };
                tiles.push(tile);
            }
        }

        Map { tiles }
    }

    pub fn in_bounds(coord: Coord) -> bool {
        coord.x < Self::WIDTH && coord.y < Self::HEIGHT
    }

    pub fn tile(&self, coord: Coord) -> &Tile {
        if Self::in_bounds(coord) {
            let idx = coord.y as usize * Self::WIDTH as usize + coord.x as usize;
            self.tiles.get(idx).unwrap_or(&Tile::WALL)
        } else {
            &Tile::WALL
        }
    }

    pub fn check_line_of_sight(&self, from: Coord, to: Coord) -> bool {
        algorithm::check_bresenhem_line(from, to, |coord| self.tile(coord).transparent)
    }
}
