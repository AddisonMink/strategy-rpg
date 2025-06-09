use std::collections::HashSet;

use super::*;
use crate::engine::*;
use macroquad::prelude::*;

/// A grid that holds light values for each coordinate in the map.
/// Each light value is a u16 representing the distance of that coordinate from edge of the nearest light source.
pub struct LightGrid {
    lights: Vec<u16>,
    colors: Vec<Color>,
    visible: Vec<bool>,
    visible_units: HashSet<UnitId>,
}

impl LightGrid {
    pub fn empty() -> Self {
        LightGrid {
            lights: vec![u16::max_value(); (Map::WIDTH * Map::HEIGHT) as usize],
            colors: vec![BLACK; (Map::WIDTH * Map::HEIGHT) as usize],
            visible: vec![false; (Map::WIDTH * Map::HEIGHT) as usize],
            visible_units: HashSet::new(),
        }
    }

    pub fn new(battle: &Battle) -> Self {
        let mut lights: Vec<u16> = vec![u16::max_value(); (Map::WIDTH * Map::HEIGHT) as usize];
        let mut colors: Vec<Color> = vec![BLACK; (Map::WIDTH * Map::HEIGHT) as usize];
        let mut visible: Vec<bool> = vec![false; (Map::WIDTH * Map::HEIGHT) as usize];
        let mut visible_units = HashSet::new();

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

        for player in battle.unit_player_iter() {
            for x in 0..Map::WIDTH {
                for y in 0..Map::HEIGHT {
                    let index = y as usize * Map::WIDTH as usize + x as usize;
                    visible[index] = battle.unit_can_see_tile(player.id, Coord::new(x, y));
                }
            }

            for npc in battle.unit_npc_iter() {
                if battle.unit_can_see_unit(player.id, npc.id) {
                    visible_units.insert(npc.id);
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
            visible_units,
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

    pub fn unit_visible(&self, unit_id: UnitId) -> bool {
        self.visible_units.contains(&unit_id)
    }
}
