use super::*;
use crate::engine::{algorithm::check_bresenhem_line, *};
use macroquad::prelude::*;

/// A grid that holds light values for each coordinate in the map.
/// Each light value is a u16 representing the distance of that coordinate from edge of the nearest light source.
pub struct LightGrid {
    pub lights: Vec<u16>,
    pub colors: Vec<Color>,
    pub visible: Vec<bool>,
}

impl LightGrid {
    pub fn empty() -> Self {
        LightGrid {
            lights: vec![u16::max_value(); (Map::WIDTH * Map::HEIGHT) as usize],
            colors: vec![BLACK; (Map::WIDTH * Map::HEIGHT) as usize],
            visible: vec![false; (Map::WIDTH * Map::HEIGHT) as usize],
        }
    }

    pub fn new(battle: &Battle, povs: impl Iterator<Item = (Coord, u16)>) -> Self {
        let mut lights: Vec<u16> = vec![u16::max_value(); (Map::WIDTH * Map::HEIGHT) as usize];
        let mut colors: Vec<Color> = vec![BLACK; (Map::WIDTH * Map::HEIGHT) as usize];
        let mut visible: Vec<bool> = vec![false; (Map::WIDTH * Map::HEIGHT) as usize];

        for (center, light) in battle.lights_iter() {
            for x in 0..Map::WIDTH {
                for y in 0..Map::HEIGHT {
                    let coord = Coord { x, y };
                    let radius = light.radius as f32;

                    if battle.map.check_line_of_sight(*center, coord) {
                        let distance = center
                            .manhattan_distance(coord)
                            .saturating_sub(light.radius);

                        let distance_from_radius = (distance as f32 - radius).max(0.0);
                        let light_alpha = (1.0 - distance_from_radius / radius).max(0.0);
                        let color = light.color.with_alpha(light_alpha);
                        let index = y as usize * Map::WIDTH as usize + x as usize;

                        lights[index] = lights[index].min(distance);
                        colors[index] = color::add_colors(colors[index], color);
                    }
                }
            }
        }

        for (coord, vision) in povs {
            for x in 0..Map::WIDTH {
                for y in 0..Map::HEIGHT {
                    let target_coord = Coord { x, y };
                    if battle.map.check_line_of_sight(coord, target_coord) {
                        let index = y as usize * Map::WIDTH as usize + x as usize;
                        let distance_from_light = lights[index];
                        let distance = coord.manhattan_distance(target_coord);
                        visible[index] =
                            visible[index] || distance_from_light <= vision || distance <= vision;
                    }
                }
            }
        }

        for color in colors.iter_mut() {
            *color = color::normalize_color(*color);
        }

        LightGrid {
            lights,
            colors,
            visible,
        }
    }

    pub fn distance_from_light(&self, coord: Coord) -> u16 {
        let index = coord.y as usize * Map::WIDTH as usize + coord.x as usize;
        self.lights[index]
    }

    pub fn color_at(&self, coord: Coord) -> Color {
        let index = coord.y as usize * Map::WIDTH as usize + coord.x as usize;
        self.colors[index]
    }

    pub fn visible(&self, coord: Coord) -> bool {
        let index = coord.y as usize * Map::WIDTH as usize + coord.x as usize;
        self.visible[index]
    }
}
