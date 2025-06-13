use macroquad::prelude::animation;

use super::light_grid::update_light_grid;
use super::player_vision::update_player_vision;
use crate::engine::*;
use crate::level_model::*;
use crate::level_system::npc_vision;

pub fn process_effects(level: &mut Level) {
    while let Some(effect) = level.effect_queue.pop_front() {
        match effect {
            Effect::UpdateLightGrid => update_light_grid(level),
            Effect::UpdateVisionGrid => update_player_vision(level),
            Effect::UpdateAllNpcVision => npc_vision::update_all_npc_vision(level),
            Effect::UpdateNpcVisionOfPlayer { player } => {
                npc_vision::update_npc_vision_of_player(level, player)
            }
            Effect::Move { entity, coord } => execute_move(level, entity, coord),
            Effect::Sleep { duration } => level.sleep_timer = Some(Timer::new(duration)),
            Effect::Damage { entity, min, max } => execute_damage(level, entity, min, max),
            Effect::Animation { animation } => level.animation_queue.push_back(animation),
            Effect::Death { entity } => execute_death(level, entity),
        }
        if level.sleep_timer.is_some() || level.animation_queue.len() > 0 {
            break;
        }
    }
}

fn execute_move(level: &mut Level, entity: Entity, coord: Coord) {
    let Some(unit) = level.units.get(&entity) else {
        return;
    };

    let Some(pos) = level.positions.get_mut(&entity) else {
        return;
    };

    pos.coord = coord;

    if unit.side == Side::Player {
        level
            .effect_queue
            .push_front(Effect::UpdateNpcVisionOfPlayer { player: entity });
    }
    level.effect_queue.push_front(Effect::UpdateVisionGrid);
}

fn execute_damage(level: &mut Level, entity: Entity, min: u16, max: u16) {
    let Some(unit) = level.units.get_mut(&entity) else {
        return;
    };

    let coord = level.positions.get(&entity).unwrap().coord;
    let damage = max;
    let text = (-(damage as i32)).to_string();

    unit.hp = unit.hp.saturating_sub(damage);
    level
        .animation_queue
        .push_back(Animation::text(coord, text, RED));

    if unit.hp == 0 {
        level
            .animation_queue
            .push_back(Animation::text(coord, "DEATH".to_string(), GRAY));
        level.animation_queue.push_back(Animation::death(entity));
        level.effect_queue.push_front(Effect::Death { entity });
    }
}

fn execute_death(level: &mut Level, entity: Entity) {
    let has_light = level.lights.contains_key(&entity);
    level.delete(entity);
    if has_light {
        level.effect_queue.push_front(Effect::UpdateLightGrid);
    }
}
