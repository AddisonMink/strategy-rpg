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

    pub fn get_tile(&self, x: u16, y: u16) -> &Tile {
        &self.tiles[(y * Self::WIDTH + x) as usize]
    }
}
