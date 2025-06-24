use macroquad::color::BLACK;

use super::state::*;
use super::world::*;
use crate::constants::*;
use crate::engine_v2::*;
use crate::util::*;

pub fn draw(world: &World, state: &State) {
    draw_world(world);
    draw_animation(world);
    draw_state(world, state);
}

fn draw_world(world: &World) {
    grid::draw_frame("Level 0");

    for coord in grid::coords_iter() {
        if !world.player_vision.tile_visible(coord) {
            continue;
        }

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

        if let Some(unit) = world
            .unit_at(coord)
            .filter(|u| world.player_vision.unit_visible(u.id()))
        {
            grid::draw_glyph(coord, unit.data().glyph.mix_color(light_color, 0.5));
        } else {
            grid::draw_glyph(coord, tile.glyph.mix_color(light_color, 0.5));
        }
    }
}

fn draw_animation(world: &World) -> Option<()> {
    let animation = world.animations.front()?;

    match &animation.kind {
        AnimationKind::Text(coord, text, color) => grid::draw_text(*coord, text.as_str(), *color),
        _ => {}
    }

    Some(())
}

fn draw_state(world: &World, state: &State) {
    match state {
        State::SelectingMove(selecting_move) => draw_selecting_move(selecting_move),
        _ => {}
    }
}

fn draw_selecting_move(selecting_move: &SelectingMove) {
    for coord in selecting_move.valid_moves.iter() {
        grid::draw_square(*coord, WHITE.with_alpha(0.5));
    }

    for coord in selecting_move.path.iter().flatten() {
        grid::draw_glyph(*coord, Glyph::new('o', WHITE));
    }

    selecting_move.cancel_button.draw();
    selecting_move.action_preview.draw();

    if let Some(tile_description) = &selecting_move.tile_description_opt {
        tile_description.draw();
    }
}
