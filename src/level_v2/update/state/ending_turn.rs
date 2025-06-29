use super::*;

pub fn transition(world: &mut World, state: &mut State) {
    *state = State::EndingTurn;
}

pub fn update(world: &mut World, state: &mut State) {
    update_unit_lights(world);

    if is_success(world) {
        *state = State::Success;
    } else {
    }

    world.end_turn();

    if is_failure(world) {
        *state = State::Failure;
    } else if is_success(world) {
        *state = State::Success;
    } else {
        world.end_turn();
        selecting_move::transition(world, state);
    }
}

fn update_unit_lights(world: &mut World) -> Option<()> {
    let unit = world.active_unit_mut()?;
    let light = unit.light.as_mut()?;

    light.radius = light.radius.saturating_sub(1);

    if light.radius == 0 {
        unit.light = None;
    }

    world.effects.push_front(Effect::UpdateNpcVision);
    world.effects.push_front(Effect::UpdatePlayerVision);
    world.effects.push_front(Effect::UpdateLightGrid);
    Some(())
}

fn is_failure(world: &World) -> bool {
    world.player_units_iter().next().is_none()
}

fn is_success(world: &World) -> bool {
    match world.goal {
        Goal::KillAllEnemies => world.npc_units_iter().next().is_none(),
        Goal::ReachExit => world
            .player_units_iter()
            .map(|p| world.map.tile(p.coord))
            .any(|t| t.goal),
    }
}
