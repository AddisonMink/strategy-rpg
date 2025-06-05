use super::Battle;
use super::UnitId;
use crate::engine::Coord;

#[derive(Debug, Clone, Copy)]
pub enum Range {
    SingleUnit { min: u16, max: u16 },
}

impl Range {
    pub fn coords_in_range(&self, battle: &Battle, origin: Coord) -> Vec<Coord> {
        match self {
            Range::SingleUnit { min, max } => {
                Range::single_unit_targets(battle, origin, *min, *max)
                    .into_iter()
                    .filter_map(|unit_id| battle.unit(unit_id).map(|unit| unit.coord))
                    .collect()
            }
        }
    }

    pub fn single_unit_targets(battle: &Battle, origin: Coord, min: u16, max: u16) -> Vec<UnitId> {
        battle
            .unit_iter()
            .filter_map(|unit| {
                let distance = origin.manhattan_distance(unit.coord);
                (distance >= min && distance <= max).then_some(unit.id)
            })
            .collect()
    }
}
