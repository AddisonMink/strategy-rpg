mod state;

use super::state::*;
use super::world::*;
use crate::util::*;
use state::update_state;

pub fn update(world: &mut World, state: &mut State, delta_time: f32) {
    loop {
        update_timer(world, delta_time);
        if world.sleep_timer.is_some() {
            break;
        }

        execute_effects(world);
        if world.sleep_timer.is_some() || !world.effects.is_empty() {
            break;
        }

        update_state(world, state, delta_time);

        if world.effects.is_empty() && world.sleep_timer.is_none() {
            break;
        }
    }
}

fn update_timer(world: &mut World, delta_time: f32) {
    if let Some(timer) = &mut world.sleep_timer {
        timer.update(delta_time);
        if timer.is_finished() {
            world.sleep_timer = None;
        }
    }
}

fn execute_effects(world: &mut World) {
    while let Some(effect) = world.effects.pop_front() {
        match effect {
            Effect::UpdateLightGrid => world.light_grid = LightGrid::new(world),
            Effect::UpdatePlayerVision => world.player_vision = PlayerVision::new(world),
            Effect::Sleep { duration } => world.sleep_timer = Some(Timer::new(duration)),
            Effect::Move { id, coord } => execute_move(world, id, coord),
        }

        if world.sleep_timer.is_some() {
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
