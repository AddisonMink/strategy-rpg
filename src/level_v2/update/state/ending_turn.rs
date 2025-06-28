use super::*;

pub fn transition(world: &mut World, state: &mut State) {
    *state = State::EndingTurn;
}

pub fn update(world: &mut World, state: &mut State) {
    let Some(unit) = world.active_unit_mut() else {
        return;
    };

    if let Some(light) = unit.light.as_mut() {
        light.radius = light.radius.saturating_sub(1);

        if light.radius == 0 {
            unit.light = None;
        }

        world.effects.push_front(Effect::UpdateNpcVision);
        world.effects.push_front(Effect::UpdatePlayerVision);
        world.effects.push_front(Effect::UpdateLightGrid);
    }

    world.end_turn();

    if world.player_units_iter().next().is_none() {
        *state = State::Failure;
    } else {
        selecting_move::transition(world, state);
    }
}
