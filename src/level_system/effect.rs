use macroquad::prelude::trace;

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
            Effect::UpdateAllNpcVisionOfPlayer { player } => {
                npc_vision::update_all_npc_vision_of_player(level, player)
            }
            Effect::UpdateNpcVisionOfAllPlayers { npc } => {
                npc_vision::update_npc_vision_of_all_players(level, npc)
            }
            Effect::Move { entity, coord } => {
                execute_move(level, entity, coord);
            }
            Effect::Sleep { duration } => level.sleep_timer = Some(Timer::new(duration)),
            Effect::Damage { entity, min, max } => execute_damage(level, entity, min, max),
            Effect::Animation { animation } => level.animation_queue.push_back(animation),
            Effect::Death { entity } => execute_death(level, entity),
            Effect::UseItem {
                entity,
                item,
                amount,
            } => execute_use_item(level, entity, item, amount),
            Effect::BreakItem { entity, item } => execute_break_item(level, entity, item),
            Effect::AddLightToEntity {
                entity,
                color,
                radius,
            } => {
                execute_add_light_to_entity(level, entity, color, radius);
            }
        }
        if level.sleep_timer.is_some() || level.animation_queue.len() > 0 {
            break;
        }
    }
}

fn execute_move(level: &mut Level, entity: Entity, coord: Coord) -> Option<()> {
    let unit = level.units.get_mut(&entity)?;
    unit.coord = coord;

    if level.lights.contains_key(&entity) {
        level.effect_queue.push_front(Effect::UpdateLightGrid);
    }

    if unit.side == Side::Player {
        level
            .effect_queue
            .push_front(Effect::UpdateAllNpcVisionOfPlayer { player: entity });
    } else {
        level
            .effect_queue
            .push_front(Effect::UpdateNpcVisionOfAllPlayers { npc: entity });
    }

    level.effect_queue.push_front(Effect::UpdateVisionGrid);
    Some(())
}

fn execute_damage(level: &mut Level, entity: Entity, min: u16, max: u16) {
    let Some(unit) = level.units.get_mut(&entity) else {
        return;
    };

    let damage = roll(min, max);
    let text = (-(damage as i32)).to_string();

    unit.hp = unit.hp.saturating_sub(damage);
    level
        .animation_queue
        .push_back(Animation::text(unit.coord, text, RED));

    if unit.hp == 0 {
        level
            .animation_queue
            .push_back(Animation::text(unit.coord, "DEATH".to_string(), GRAY));
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

fn execute_use_item(level: &mut Level, entity: Entity, item: ItemId, amount: u16) {
    let Some(inventory) = level.inventories.get_mut(&entity) else {
        return;
    };

    if let Some(item) = inventory.items.get_mut(&item) {
        item.uses = item.uses.saturating_sub(amount);
        if item.uses == 0 {
            level.effect_queue.push_front(Effect::BreakItem {
                entity,
                item: item.id,
            });
        }
    }
}

fn execute_break_item(level: &mut Level, entity: Entity, item: ItemId) {
    let Some(unit) = level.units.get(&entity) else {
        return;
    };

    let Some(inventory) = level.inventories.get_mut(&entity) else {
        return;
    };

    let Some(item_name) = inventory.items.get(&item).map(|i| i.name.clone()) else {
        return;
    };

    inventory.items.remove(&item);
    level.animation_queue.push_back(Animation::panel_text(
        unit.coord,
        format!("{} broke!", item_name.as_str()),
    ));
}

fn execute_add_light_to_entity(
    level: &mut Level,
    entity: Entity,
    color: Color,
    radius: u16,
) -> Option<()> {
    let unit = level.units.get_mut(&entity)?;
    unit.light = Some(Light { radius, color });
    level.effect_queue.push_front(Effect::UpdateLightGrid);
    Some(())
}

fn roll(low: u16, high: u16) -> u16 {
    let roll1 = gen_range(low, high + 1) as f32;
    let roll2 = gen_range(low, high + 1) as f32;
    let roll = (roll1 + roll2) / 2.0;
    roll.round() as u16
}
