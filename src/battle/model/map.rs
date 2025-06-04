use crate::engine::*;

use super::tile::Tile;

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
        "#..#..#..#",
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
}
