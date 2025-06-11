use super::map::Map;
use crate::engine::*;

pub struct LightGrid {
    pub distances_from_light: Vec<u16>,
    pub light_colors: Vec<Color>,
}

impl LightGrid {
    pub fn empty() -> Self {
        let size = (Map::WIDTH * Map::HEIGHT) as usize;
        LightGrid {
            distances_from_light: vec![u16::max_value(); size],
            light_colors: vec![BLACK; size],
        }
    }

    pub fn distance_from_light(&self, coord: Coord) -> u16 {
        if Map::in_bounds(coord) {
            self.distances_from_light[Self::index(coord)]
        } else {
            u16::max_value()
        }
    }

    pub fn light_color(&self, coord: Coord) -> Color {
        if Map::in_bounds(coord) {
            self.light_colors[Self::index(coord)]
        } else {
            BLACK
        }
    }

    fn index(coord: Coord) -> usize {
        (coord.y as usize * Map::WIDTH as usize) + coord.x as usize
    }
}
