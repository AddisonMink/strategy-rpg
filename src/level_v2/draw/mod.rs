use macroquad::color::BLACK;

use super::state::*;
use super::world::*;
use crate::constants::*;
use crate::engine::color::mix_color;
use crate::util::*;

pub fn draw(world: &World, state: &State) {
    draw_world(world);
    draw_state(world, state);
}

fn draw_world(world: &World) {
    grid::draw_frame("Level 0");

    for coord in grid::coords_iter() {
        let tile = world.map.tile(coord);
        let light_distance = world.light_grid.distance_from_light(coord);

        let light_color = if light_distance == 0 {
            world.light_grid.light_color(coord)
        } else {
            BLACK
        };

        if let Some(bg_color) = tile.bg_color {
            grid::draw_square(coord, mix_color(bg_color, light_color, 0.5));
        }

        if let Some(unit) = world.unit_at(coord) {
            grid::draw_glyph(coord, unit.data().glyph.mix_color(light_color, 0.5));
        } else {
            grid::draw_glyph(coord, tile.glyph.mix_color(light_color, 0.5));
        }
    }
}

fn draw_state(world: &World, state: &State) {}
