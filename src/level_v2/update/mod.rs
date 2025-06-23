mod state;

use super::state::*;
use super::world::*;
use crate::util::*;
use state::update_state;

pub fn update(world: &mut World, state: &mut State, delta_time: f32) {
    loop {
        update_animations(world, delta_time);
        if !world.animations.is_empty() {
            break;
        }

        execute_effects(world);
        if !world.animations.is_empty() {
            break;
        }

        update_state(world, state, delta_time);
        if !world.animations.is_empty() {
            break;
        }

        match state {
            State::SelectingMove(..) => break,
            _ => {}
        }
    }
}

fn update_animations(world: &mut World, delta_time: f32) {
    if let Some(animation) = world.animations.front_mut() {
        animation.timer.update(delta_time);
        if animation.timer.is_finished() {
            world.animations.pop_front();
        }
    }
}

fn execute_effects(world: &mut World) {
    while let Some(effect) = world.effects.pop_front() {
        match effect {
            Effect::UpdateLightGrid => world.light_grid = LightGrid::new(world),
            Effect::UpdatePlayerVision => world.player_vision = PlayerVision::new(world),
            Effect::Sleep { duration } => world.animations.push_front(Animation::sleep(duration)),
            Effect::Move { id, coord } => execute_move(world, id, coord),
        }

        if !world.animations.is_empty() {
            break;
        }
    }
}

fn execute_move(world: &mut World, id: UnitId, coord: Coord) {
    let Some(unit) = world.unit_mut(id) else {
        return;
    };

    unit.coord = coord;
    world.effects.push_front(Effect::UpdatePlayerVision);
    world.effects.push_front(Effect::UpdateLightGrid);
}
