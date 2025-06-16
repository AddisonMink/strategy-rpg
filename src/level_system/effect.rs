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
            Effect::Death { entity } => {
                execute_death(level, entity);
            }
            Effect::UseItem {
                entity,
                item,
                amount,
            } => {
                execute_use_item(level, entity, item, amount);
            }
            Effect::BreakItem { entity, item } => {
                execute_break_item(level, entity, item);
            }
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

fn execute_move(level: &mut Level, id: UnitId, coord: Coord) -> Option<()> {
    let unit = level.units.get_mut(&id)?;
    unit.coord = coord;

    if unit.light.is_some() {
        level.effect_queue.push_front(Effect::UpdateLightGrid);
    }

    if unit.side == Side::Player {
        level
            .effect_queue
            .push_front(Effect::UpdateAllNpcVisionOfPlayer { player: id });
    } else {
        level
            .effect_queue
            .push_front(Effect::UpdateNpcVisionOfAllPlayers { npc: id });
    }

    level.effect_queue.push_front(Effect::UpdateVisionGrid);
    Some(())
}

fn execute_damage(level: &mut Level, entity: UnitId, min: u16, max: u16) {
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

fn execute_death(level: &mut Level, id: UnitId) -> Option<()> {
    let has_light = level.units.get(&id)?.light.is_some();
    level.delete_unit(id);
    if has_light {
        level.effect_queue.push_front(Effect::UpdateLightGrid);
    }
    Some(())
}

fn execute_use_item(level: &mut Level, entity: UnitId, item: ItemId, amount: u16) -> Option<()> {
    let unit = level.units.get_mut(&entity)?;
    let item = unit.items.get_mut(&item)?;

    item.uses = item.uses.saturating_sub(amount);
    if item.uses == 0 {
        level.effect_queue.push_front(Effect::BreakItem {
            entity,
            item: item.id,
        });
    }
    Some(())
}

fn execute_break_item(level: &mut Level, entity: UnitId, item: ItemId) -> Option<()> {
    let unit = level.units.get_mut(&entity)?;
    let item_name = unit.items.get(&item)?.name.clone();

    unit.items.remove(&item);
    level.animation_queue.push_back(Animation::panel_text(
        unit.coord,
        format!("{} broke!", item_name.as_str()),
    ));
    Some(())
}

fn execute_add_light_to_entity(
    level: &mut Level,
    entity: UnitId,
    color: Color,
    radius: u16,
) -> Option<()> {
    let unit = level.units.get_mut(&entity)?;
    unit.light = Some(Light { radius, color });
    level.effect_queue.push_front(Effect::UpdateVisionGrid);
    level.effect_queue.push_front(Effect::UpdateLightGrid);
    Some(())
}

fn roll(low: u16, high: u16) -> u16 {
    let roll1 = gen_range(low, high + 1) as f32;
    let roll2 = gen_range(low, high + 1) as f32;
    let roll = (roll1 + roll2) / 2.0;
    roll.round() as u16
}
