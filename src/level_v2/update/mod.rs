use std::collections::HashSet;

use super::state::*;
use super::world::*;
use crate::engine_v2::*;
use crate::util::*;

pub fn update(world: &mut World, state: &mut State, delta_time: f32) {
    execute_effects(world);
    update_state(world, state);
}

fn execute_effects(world: &mut World) {
    while let Some(effect) = world.effects.pop_front() {
        match effect {
            Effect::UpdateLightGrid => world.light_grid = LightGrid::new(world),
        }
    }
}

fn update_state(world: &mut World, state: &mut State) {
    match state {
        State::Starting(..) => {
            world.effects.push_back(Effect::UpdateLightGrid);
        }
        State::SelectingMove(..) => {}
    }
}
