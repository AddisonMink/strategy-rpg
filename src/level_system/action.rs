use crate::engine::*;
use crate::level_model::*;
use std::collections::HashSet;

pub fn find_target_coords(
    level: &Level,
    entity: Entity,
    origin: Coord,
    action: &Action,
) -> HashSet<Coord> {
    match action.range {
        Range::SelfRange => HashSet::from([origin]),
        Range::SingleUnit { min, max } => {
            single_unit_range_targets(level, entity, origin, min, max)
                .iter()
                .map(|e| level.positions.get(e).unwrap().coord)
                .collect()
        }
    }
}

pub fn single_unit_range_targets(
    level: &Level,
    entity: Entity,
    origin: Coord,
    min_range: u16,
    max_range: u16,
) -> HashSet<Entity> {
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
