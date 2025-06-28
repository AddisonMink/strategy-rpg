use std::collections::HashSet;

use super::*;

#[derive(Debug, Clone, Copy)]
pub enum ActionRange {
    SelfRange,
    Enemy { min_range: u16, max_range: u16 },
}

#[derive(Debug, Clone, Copy)]
pub enum ActionEffect {
    Attack,
    Damage { min: u16, max: u16 },
    Light { light: Light },
}

#[derive(Debug, Clone, Copy)]
pub struct Action {
    pub name: ShortString,
    pub cost: u16,
    pub range: ActionRange,
    pub effects: ShortList<ActionEffect>,
}

impl Action {
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
            ActionRange::SelfRange => Some(ActionTargets::SelfTarget(unit.id())),
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
}

pub enum ActionTargets {
    SelfTarget(UnitId),
    EnemyTargets(HashSet<UnitId>),
}

pub enum ActionTarget {
    SelfTarget(UnitId),
    SingleEnemy(UnitId),
}
