use macroquad::color;
use macroquad::color::BLACK;
use macroquad::text;

use super::state::*;
use super::world::*;
use crate::constants::*;
use crate::engine::coord;
use crate::engine_v2::*;
use crate::util::*;

pub fn draw(world: &World, state: &State) {
    draw_world(world);
    draw_animation(world);
    draw_state(world, state);
}

fn draw_world(world: &World) {
    grid::draw_frame("Level 0");

    let animating_unit = world.animations.front().and_then(|a| a.unit_id());

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
            .filter(|u| animating_unit != Some(u.id()))
        {
            grid::draw_glyph(coord, unit.data().glyph.mix_color(light_color, 0.5));
        } else {
            grid::draw_glyph(coord, tile.glyph.mix_color(light_color, 0.5));
        }
    }
}

fn draw_animation(world: &World) -> Option<()> {
    let animation = world.animations.front()?;
    let progress = animation.timer.progress();

    match &animation.kind {
        AnimationKind::Text(coord, text, color) => grid::draw_text(*coord, text.as_str(), *color),
        AnimationKind::FadingRisingText(coord, text, color, max_offset) => {
            draw_fading_rising_text(*coord, &text.as_str(), *color, *max_offset, progress)
        }
        AnimationKind::UnitAnimation(unit_animation) => match unit_animation.kind {
            UnitAnimationKind::Attack(dir) => {
                draw_attack_animation(world, unit_animation.id, dir, progress);
            }
            UnitAnimationKind::Death => draw_death_animation(world, unit_animation.id, progress),
        },
        _ => {}
    }

    Some(())
}

fn draw_fading_rising_text(coord: Coord, text: &str, color: Color, max_offset: f32, progress: f32) {
    let offset = (-progress) * max_offset;
    let alpha = 1.0 - progress;
    let faded_color = color.with_alpha(alpha);

    grid::draw_text_with_offset(coord, text, faded_color, (0.0, offset));
}

fn draw_attack_animation(world: &World, id: UnitId, dir: Direction, progress: f32) {
    let Some(unit) = world.unit(id) else { return };

    if !world.player_vision.unit_visible(unit.id()) {
        return;
    };

    let light_color = world.light_grid.light_color(unit.coord);
    let glyph = unit.data().glyph.mix_color(light_color, 0.5);
    let t = (progress * std::f32::consts::PI).sin() * TILE_SIZE / 2.0;

    let offset = match dir {
        Direction::Up => (0.0, -t),
        Direction::Down => (0.0, t),
        Direction::Left => (-t, 0.0),
        Direction::Right => (t, 0.0),
    };

    grid::draw_glyph_with_offset(unit.coord, glyph, offset);
}

fn draw_death_animation(world: &World, id: UnitId, progress: f32) {
    let Some(unit) = world.unit(id) else { return };

    if !world.player_vision.unit_visible(unit.id()) {
        return;
    };

    let light_color = world.light_grid.light_color(unit.coord);
    let alpha = 1.0 - progress;

    let glyph = unit
        .data()
        .glyph
        .mix_color(light_color, 0.5)
        .with_alpha(alpha);

    grid::draw_glyph(unit.coord, glyph);
}

fn draw_state(world: &World, state: &State) {
    match state {
        State::SelectingMove(selecting_move) => draw_selecting_move(selecting_move),
        State::SelectingAction(selecting_action) => draw_selecting_action(selecting_action),
        State::SelectingEnemyTarget(selecting_target) => {
            draw_selecting_enemy_target(selecting_target)
        }
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

    if let Some(unit_description) = &selecting_move.unit_description_opt {
        unit_description.draw();
    }

    if let Some(action_description) = &selecting_move.action_description_opt {
        action_description.draw();
    }
}

pub fn draw_selecting_action(selecting_action: &SelectingAction) {
    selecting_action.cancel_button.draw();
    selecting_action.action_list.draw();

    if let Some(tile_description) = &selecting_action.tile_description_opt {
        tile_description.draw();
    }

    if let Some(unit_description) = &selecting_action.unit_description_opt {
        unit_description.draw();
    }

    if let Some(action_description) = &selecting_action.action_description_opt {
        action_description.draw();
    }
}

pub fn draw_selecting_enemy_target(selecting_target: &SelectingEnemyTarget) {
    selecting_target.cancel_button.draw();
    selecting_target.action_description.draw();

    if let Some(tile_description) = &selecting_target.tile_description_opt {
        tile_description.draw();
    }

    if let Some(unit_description) = &selecting_target.unit_description_opt {
        unit_description.draw();
    }

    for coord in selecting_target.targets.keys() {
        grid::draw_square(*coord, WHITE.with_alpha(0.5));
    }

    if let Some(selected_target) = selecting_target.selected_target {
        grid::draw_square(selected_target, WHITE.with_alpha(0.5));
    }
}
