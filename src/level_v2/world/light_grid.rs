use super::map::Map;
use crate::engine_v2::*;
use crate::level_v2::world::World;
use crate::util::*;

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
                    let light_alpha = (1.0 - distance_from_radius / radius).max(0.0);
                    let color = light.color.with_alpha(light_alpha);
                    let index = Self::index(coord);

                    lights[index] = lights[index].min(distance);
                    colors[index] = add_colors(colors[index], color);
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
