use super::Battle;
use super::EffectTemplate;
use super::Range;
use crate::battle::model::UnitId;
use crate::engine::*;

#[derive(Debug, Clone, Copy)]
pub struct Action {
    pub name: ShortString,
    pub range: Range,
    pub effects: ShortList<EffectTemplate>,
}

impl Action {
    pub const ATTACK: Self = Self {
        name: ShortString::new("Attack"),
        range: Range::SingleUnit { min: 1, max: 1 },
        effects: ShortList::new(&[EffectTemplate::Damage { min: 1, max: 5 }]),
    };

    pub fn has_valid_targets(&self, battle: &Battle, unit_id: UnitId, origin: Coord) -> bool {
        match self.range {
            Range::SingleUnit { min, max } => battle.unit_iter().any(|unit| {
                let distance = origin.manhattan_distance(unit.coord);
                distance >= min && distance <= max && unit.id != unit_id
            }),
        }
    }
}
