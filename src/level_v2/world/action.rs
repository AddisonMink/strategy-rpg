use std::collections::HashSet;

use super::*;

#[derive(Debug, Clone, Copy)]
pub enum ActionRange {
    Enemy { min_range: u16, max_range: u16 },
}

#[derive(Debug, Clone, Copy)]
pub enum ActionEffect {
    Attack,
    Damage { min: u16, max: u16 },
}

#[derive(Debug, Clone, Copy)]
pub struct Action {
    pub name: ShortString,
    pub range: ActionRange,
    pub effects: ShortList<ActionEffect>,
}

impl Action {
    pub const ATTACK: Self = Self {
        name: ShortString::new("Attack"),
        range: ActionRange::Enemy {
            min_range: 1,
            max_range: 1,
        },
        effects: ShortList::new(&[
            ActionEffect::Attack,
            ActionEffect::Damage { min: 1, max: 3 },
        ]),
    };

    pub fn find_targets(&self, world: &World, unit: &Unit) -> Option<ActionTargets> {
        let origin = unit.coord;
        self.find_targets_from(world, unit, origin)
    }

    pub fn find_targets_from(
        &self,
        world: &World,
        unit: &Unit,
        origin: Coord,
    ) -> Option<ActionTargets> {
        match self.range {
            ActionRange::Enemy {
                min_range,
                max_range,
            } => self.find_enemy_targets(world, unit, origin, min_range, max_range),
        }
    }

    fn find_enemy_targets(
        &self,
        world: &World,
        unit: &Unit,
        origin: Coord,
        min_range: u16,
        max_range: u16,
    ) -> Option<ActionTargets> {
        let targets: HashSet<UnitId> = world
            .npc_units_iter()
            .filter_map(|npc| {
                let dist = origin.manhattan_distance(npc.coord);
                let in_range = dist >= min_range && dist <= max_range;
                let visible = world.unit_can_see_unit(unit.id(), npc.id());
                (in_range && visible).then_some(npc.id())
            })
            .collect();

        (!targets.is_empty()).then_some(ActionTargets::EnemyTargets(targets))
    }

    pub fn compile_single_enemy_action(
        &self,
        world: &World,
        unit: &Unit,
        enemy_id: UnitId,
    ) -> VecDeque<Effect> {
        let mut effects = VecDeque::new();

        for effect in self.effects.iter() {
            match effect {
                ActionEffect::Attack => {
                    if let Some(enemy) = world.unit(enemy_id) {
                        let dir = unit.coord.direction_to(enemy.coord).unwrap();
                        let animation = Animation::attack(unit.id(), dir);
                        let effect = Effect::Animate { animation };
                        effects.push_back(effect);
                    }
                }
                ActionEffect::Damage { min, max } => {
                    let effect = Effect::Damage {
                        id: enemy_id,
                        min: *min,
                        max: *max,
                    };
                    effects.push_back(effect);
                }
            }
        }

        effects
    }
}

pub enum ActionTargets {
    EnemyTargets(HashSet<UnitId>),
}

pub enum ActionTarget {
    SingleEnemy(UnitId),
}
