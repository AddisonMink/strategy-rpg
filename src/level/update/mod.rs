mod state;

use super::state::*;
use super::world::*;
use crate::util::*;
use macroquad::color::RED;
use macroquad::color::YELLOW;
use macroquad::prelude::trace;
use macroquad::rand::gen_range;
use state::update_state;

pub fn update(world: &mut World, state: &mut State, delta_time: f32) -> Option<LevelResult> {
    loop {
        update_animations(world, delta_time);
        if !world.animations.is_empty() {
            break;
        }

        execute_effects(world);
        if !world.animations.is_empty() {
            break;
        }

        update_state(world, state);
        if !world.animations.is_empty() {
            break;
        }

        match state {
            State::SelectingMove(..) => break,
            State::SelectingAction(..) => break,
            State::SelectingEnemyTarget(..) => break,
            State::Failure => break,
            State::Success => break,
            State::Ending(..) => break,
            _ => {}
        }
    }

    if let State::Ending(result) = state {
        Some(*result)
    } else {
        None
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
            Effect::UpdateNpcVision => execute_update_npc_vision(world),
            Effect::Sleep { duration } => world.animations.push_front(Animation::sleep(duration)),
            Effect::Move { id, coord } => execute_move(world, id, coord),
            Effect::Damage { id, min, max } => execute_damage(world, id, min, max),
            Effect::Kill { id } => execute_kill(world, id),
            Effect::ConsumeCharge {
                id,
                item_id,
                amount,
            } => execute_consume_charge(world, id, item_id, amount),
            Effect::AddUnitLight { id, light } => execute_add_unit_light(world, id, light),
            Effect::Animate { animation } => world.animations.push_back(animation),
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
    world.effects.push_front(Effect::UpdateNpcVision);
    world.effects.push_front(Effect::UpdatePlayerVision);
    world.effects.push_front(Effect::UpdateLightGrid);
}

fn execute_update_npc_vision(world: &mut World) {
    let player_ids = world
        .player_units_iter()
        .map(|player| player.id())
        .collect::<Vec<_>>();

    let npc_ids = world
        .npc_units_iter()
        .map(|npc| npc.id())
        .collect::<Vec<_>>();

    for npc_id in npc_ids.iter() {
        for moved_player_id in player_ids.iter() {
            update_npc_vision_of_player(world, *npc_id, *moved_player_id);
        }
    }
}

fn update_npc_vision_of_player(
    world: &mut World,
    npc_id: UnitId,
    moved_player_id: UnitId,
) -> Option<()> {
    let player_coord = world.unit(moved_player_id)?.coord;
    let npc_coord = world.unit(npc_id)?.coord;
    let visible = world.unit_can_see_unit(npc_id, moved_player_id);
    let visible_to_player = world.unit_can_see_unit(moved_player_id, npc_id);
    let memory = &world.unit(npc_id)?.memory;
    let player_seen = visible && !memory.visible_players.contains(&moved_player_id);
    let player_lost = !visible && memory.visible_players.contains(&moved_player_id);

    // If the NPC has lost sight of the player, it remembers the last known position.
    if player_lost {
        let npc = world.unit_mut(npc_id)?;
        npc.memory.visible_players.remove(&moved_player_id);
        npc.memory.last_seen_player = Some((moved_player_id, player_coord));

        if visible_to_player {
            world
                .animations
                .push_front(Animation::text(npc_coord, ShortString::new("?"), YELLOW));
        }
    }
    // If the NPC sees the player for the first time, it updates its memory.
    else if player_seen {
        let npc = world.unit_mut(npc_id)?;
        npc.memory.visible_players.insert(moved_player_id);

        if visible_to_player {
            world
                .animations
                .push_front(Animation::text(npc_coord, ShortString::new("!"), RED));
        }
    }

    Some(())
}

fn execute_damage(world: &mut World, id: UnitId, min: u16, max: u16) {
    let Some(unit) = world.unit_mut(id) else {
        return;
    };

    let damage = roll(min, max);
    let damage_str = ShortString::new(&(-(damage as i16)).to_string());
    let damage_animation = Animation::text(unit.coord, damage_str, RED);
    let dead = unit.hp <= damage;

    unit.hp = unit.hp.saturating_sub(damage);
    world.animations.push_back(damage_animation);

    if dead {
        world.animations.push_back(Animation::death(id));
        world.effects.push_front(Effect::Kill { id });
    }
}

fn execute_kill(world: &mut World, id: UnitId) {
    world.remove_unit(id);
    world.effects.push_front(Effect::UpdateNpcVision);
    world.effects.push_front(Effect::UpdatePlayerVision);
    world.effects.push_front(Effect::UpdateLightGrid);
}

fn execute_consume_charge(world: &mut World, id: UnitId, item_id: ItemId, amount: u16) {
    let Some(unit) = world.unit_mut(id) else {
        return;
    };

    let coord = unit.coord;

    let broken = unit
        .items
        .get_mut(&item_id)
        .map(|item| {
            item.charges = item.charges.saturating_sub(amount);
            (item.charges == 0)
        })
        .unwrap_or(false);

    if broken {
        unit.items.remove(&item_id);

        world
            .animations
            .push_back(Animation::text(coord, ShortString::new("broken"), RED));
    }
}

fn execute_add_unit_light(world: &mut World, id: UnitId, light: Light) {
    if let Some(unit) = world.unit_mut(id) {
        unit.light = Some(light);
        world.effects.push_front(Effect::UpdateNpcVision);
        world.effects.push_front(Effect::UpdatePlayerVision);
        world.effects.push_front(Effect::UpdateLightGrid);
    }
}

fn roll(min: u16, max: u16) -> u16 {
    let roll1 = gen_range(min, max + 1) as f32;
    let roll2 = gen_range(min, max + 1) as f32;
    let avg = (roll1 + roll2) / 2.0;
    avg.round() as u16
}
