use std::f32::consts::E;

use macroquad::color;

use super::map::Map;
use crate::engine::*;
use crate::level::world::World;
use crate::util::*;

pub struct LightGrid {
    distances_from_light: Vec<u16>,
    light_colors: Vec<Color>,
}

impl LightGrid {
    pub fn empty() -> Self {
        let size = (Map::WIDTH * Map::HEIGHT) as usize;
        LightGrid {
            distances_from_light: vec![u16::max_value(); size],
            light_colors: vec![BLACK; size],
        }
    }

    pub fn new(world: &World) -> Self {
        let mut lights: Vec<u16> = vec![u16::max_value(); (Map::WIDTH * Map::HEIGHT) as usize];
        let mut colors: Vec<Color> = vec![BLACK; (Map::WIDTH * Map::HEIGHT) as usize];

        for (origin, light) in world.lights_iter() {
            for coord in grid::coords_iter() {
                let radius = light.radius as f32;

                if world.map.check_line_of_sight(origin, coord) {
                    let distance = origin
                        .manhattan_distance(coord)
                        .saturating_sub(light.radius);

                    let distance_from_radius = (distance as f32 - radius).max(0.0);
                    let index = Self::index(coord);

                    lights[index] = lights[index].min(distance);

                    if distance_from_radius <= 0.0 {
                        let old_color = colors[index];
                        if old_color == BLACK {
                            colors[index] = light.color;
                        } else {
                            colors[index] = mix_color(old_color, light.color, 0.5);
                        }
                    }
                }
            }
        }

        Self {
            distances_from_light: lights,
            light_colors: colors,
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
