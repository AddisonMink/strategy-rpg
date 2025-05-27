use crate::algorithm;
use crate::coord::Coord;
use crate::tile::Tile;

pub struct Map {
    tiles: Vec<Tile>,
}

impl Map {
    pub const WIDTH: u16 = 16;
    pub const HEIGHT: u16 = 7;

    const FLOOR_PLAN: [&str; 7] = [
        // Example floor plan, can be modified as needed
        "################",
        "#..............#",
        "#.############.#",
        "#..............#",
        "#.############.#",
        "#..............#",
        "################",
    ];

    pub fn new() -> Self {
        let mut tiles = Vec::with_capacity((Self::WIDTH * Self::HEIGHT) as usize);

        for row in Self::FLOOR_PLAN.iter() {
            for ch in row.chars() {
                let tile = match ch {
                    '#' => Tile::WALL,
                    '.' => Tile::FLOOR,
                    _ => Tile::FLOOR,
                };
                tiles.push(tile);
            }
        }

        Map { tiles }
    }

    pub fn tile(&self, coord: Coord) -> &Tile {
        &self.tiles[(coord.y * Self::WIDTH + coord.x) as usize]
    }

    pub fn check_line_of_sight(&self, from: Coord, to: Coord) -> bool {
        algorithm::check_bresenhem_line(from, to, |coord| self.tile(coord).transparent)
    }
}
