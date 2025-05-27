use crate::model::*;
use crate::render::{add_colors, normalize_color};
use crate::util::*;
use macroquad::prelude::*;

/// A grid that holds light values for each coordinate in the map.
/// Each light value is a u16 representing the distance of that coordinate from edge of the nearest light source.
pub struct LightGrid {
    pub lights: Vec<u16>,
    pub colors: Vec<Color>,
}

impl LightGrid {
    pub fn new(map: &Map, entities: &Entities) -> Self {
        let mut lights: Vec<u16> = vec![u16::max_value(); (Map::WIDTH * Map::HEIGHT) as usize];
        let mut colors: Vec<Color> = vec![BLACK; (Map::WIDTH * Map::HEIGHT) as usize];

        for light in entities.lights.values() {
            if let Some(position) = entities.positions.get(&light.id) {
                let radius = light.radius;
                let center = position.coord;
                for x in 0..Map::WIDTH {
                    for y in 0..Map::HEIGHT {
                        let coord = Coord { x, y };
                        if map.check_line_of_sight(center, coord) {
                            let distance = center.manhattan_distance(coord).saturating_sub(radius);
                            let distance_from_radius = (distance as f32 - radius as f32).max(0.0);
                            let light_alpha = (1.0 - distance_from_radius / radius as f32).max(0.0);
                            let color = light.color.with_alpha(light_alpha);
                            let index = y as usize * Map::WIDTH as usize + x as usize;

                            lights[index] = lights[index].min(distance);
                            colors[index] = add_colors(colors[index], color);
                        }
                    }
                }
            }
        }

        for color in colors.iter_mut() {
            *color = normalize_color(*color);
        }

        LightGrid { lights, colors }
    }

    pub fn distance_from_light(&self, coord: Coord) -> u16 {
        let index = coord.y as usize * Map::WIDTH as usize + coord.x as usize;
        self.lights[index]
    }

    pub fn color_at(&self, coord: Coord) -> Color {
        let index = coord.y as usize * Map::WIDTH as usize + coord.x as usize;
        self.colors[index]
    }
}
