use std::collections::HashSet;

use super::*;

#[derive(Debug, Clone, Copy)]
pub enum ActionRange {
    Enemy { min_range: u16, max_range: u16 },
}

#[derive(Debug, Clone, Copy)]
pub enum ActionEffect {
    Damage { min: u16, max: u16 },
}

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
            max_range: 3,
        },
        effects: ShortList::new(&[ActionEffect::Damage { min: 1, max: 3 }]),
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
            } => self.find_enemy_targets(world, unit, origin),
        }
    }

    fn find_enemy_targets(
        &self,
        world: &World,
        unit: &Unit,
        origin: Coord,
    ) -> Option<ActionTargets> {
        let mut targets = HashSet::new();
        Some(ActionTargets::EnemyTargets(targets))
    }
}

pub enum ActionTargets {
    EnemyTargets(HashSet<UnitId>),
}
