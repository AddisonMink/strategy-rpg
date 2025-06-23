mod state;

use super::state::*;
use super::world::*;
use state::update_state;

pub fn update(world: &mut World, state: &mut State, delta_time: f32) {
    loop {
        execute_effects(world);
        update_state(world, state, delta_time);
        if world.effects.is_empty() {
            break;
        }
    }
}

fn execute_effects(world: &mut World) {
    while let Some(effect) = world.effects.pop_front() {
        match effect {
            Effect::UpdateLightGrid => world.light_grid = LightGrid::new(world),
            Effect::UpdatePlayerVision => world.player_vision = PlayerVision::new(world),
        }
    }
}
