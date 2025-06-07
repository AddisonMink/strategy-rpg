use super::*;
use crate::battle::model::Animation;
use crate::engine::*;
use std::collections::VecDeque;

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
        effects: ShortList::new(&[
            EffectTemplate::EnqueueAttackAnimation,
            EffectTemplate::Damage { min: 1, max: 5 },
        ]),
    };

    pub fn has_valid_targets(&self, battle: &Battle, unit_id: UnitId, origin: Coord) -> bool {
        match self.range {
            Range::SingleUnit { min, max } => battle.unit_iter().any(|unit| {
                let distance = origin.manhattan_distance(unit.coord);
                distance >= min && distance <= max && unit.id != unit_id
            }),
        }
    }

    pub fn compile_single_unit_target_effects(
        &self,
        battle: &Battle,
        unit: UnitId,
        target: UnitId,
    ) -> VecDeque<Effect> {
        let unit = battle.unit(unit).expect("Unit must exist");
        let target_unit = battle.unit(target).expect("Target unit must exist");

        self.effects
            .as_slice()
            .into_iter()
            .map(|e| match e {
                EffectTemplate::Damage { min, max } => Effect::Damage {
                    min: *min,
                    max: *max,
                    target,
                },
                EffectTemplate::EnqueueAttackAnimation => {
                    if let Some(direction) = unit.coord.direction_to(target_unit.coord) {
                        Effect::QueueAnimation {
                            animation: Animation::attack(unit.id, direction),
                        }
                    } else {
                        Effect::Noop
                    }
                }
            })
            .collect()
    }
}
