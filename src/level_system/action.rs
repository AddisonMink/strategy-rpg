use crate::engine::*;
use crate::level_model::*;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn valid_player_actions(level: &Level) -> Vec<ItemAction> {
    let entity = level.turn_queue.front().unwrap();
    let origin = level.units.get(entity).unwrap().coord;
    let actions = player_actions(level, origin);
    actions
        .into_iter()
        .filter_map(|(action, valid)| if valid { Some(action) } else { None })
        .collect()
}

pub fn player_actions(level: &Level, origin: Coord) -> Vec<(ItemAction, bool)> {
    let unit = level.active_unit().unwrap();
    let mut actions = Vec::new();

    for item in unit.items.values() {
        for action in item.actions.as_slice() {
            if action.cost <= item.uses {
                let item_action = ItemAction {
                    item_id: item.id,
                    item_name: item.name.clone(),
                    item_color: item.color,
                    uses_max: item.uses_max,
                    uses: item.uses,
                    action: action.clone(),
                };
                let valid = has_valid_targets(level, unit.entity, origin, &item_action.action);
                actions.push((item_action, valid));
            }
        }
    }

    actions
}

pub fn has_valid_targets(level: &Level, entity: UnitId, origin: Coord, action: &Action) -> bool {
    let target_coords = find_target_coords(level, entity, origin, action);
    !target_coords.is_empty()
}

pub fn find_target_coords(
    level: &Level,
    entity: UnitId,
    origin: Coord,
    action: &Action,
) -> HashSet<Coord> {
    match action.range {
        Range::SelfRange => HashSet::from([origin]),
        Range::SingleUnit { min, max } => {
            single_unit_range_targets(level, entity, origin, min, max)
                .iter()
                .map(|e| level.units.get(e).unwrap().coord)
                .collect()
        }
    }
}

pub fn single_unit_range_targets(
    level: &Level,
    entity: UnitId,
    origin: Coord,
    min_range: u16,
    max_range: u16,
) -> HashSet<UnitId> {
    let mut coords = HashSet::new();
    for dy in origin.y.saturating_sub(max_range)..=(origin.y + max_range) {
        for dx in origin.x.saturating_sub(max_range)..=(origin.x + max_range) {
            let coord = Coord::new(dx, dy);
            let distance = origin.manhattan_distance(coord);

            let Some(target) = level.unit_at(coord) else {
                continue;
            };

            if target.entity == entity {
                continue;
            }

            let visible = level.player_vision.entity_visible(target.entity);
            if visible && distance >= min_range && distance <= max_range {
                coords.insert(target.entity);
            }
        }
    }
    coords
}

pub fn compile_self_action(level: &Level, action: &ItemAction, entity: UnitId) -> VecDeque<Effect> {
    compile_single_unit_action(level, action, entity, entity)
}

pub fn compile_single_unit_action(
    level: &Level,
    action: &ItemAction,
    actor: UnitId,
    target: UnitId,
) -> VecDeque<Effect> {
    let mut effect_queue = VecDeque::new();

    for effect in action.action.effects.as_slice() {
        match effect {
            EffectTemplate::AttackAnimation => {
                let Some(actor_unit) = level.units.get(&actor) else {
                    continue;
                };
                let Some(target_unit) = level.units.get(&target) else {
                    continue;
                };
                let Some(direction) = actor_unit.coord.direction_to(target_unit.coord) else {
                    continue;
                };
                effect_queue.push_back(Effect::Animation {
                    animation: Animation::attack(actor, direction),
                });
            }
            EffectTemplate::Damage { min, max } => {
                effect_queue.push_back(Effect::Damage {
                    entity: target,
                    min: *min,
                    max: *max,
                });
            }
            EffectTemplate::AddLightToEntity { color, radius } => {
                effect_queue.push_back(Effect::AddLightToEntity {
                    entity: target,
                    color: *color,
                    radius: *radius,
                });
            }
        }
    }

    effect_queue.push_back(Effect::UseItem {
        entity: actor,
        item: action.item_id,
        amount: action.action.cost,
    });

    effect_queue
}
