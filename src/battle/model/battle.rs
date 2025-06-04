use super::Map;
use super::*;
use crate::engine::*;
use std::collections::{HashMap, VecDeque};

pub struct Battle {
    pub map: Map,
    units: HashMap<UnitId, Unit>,
    next_id: UnitId,
    turn_queue: VecDeque<UnitId>,
    pub state: BattleState,
}

impl Battle {
    pub fn new(map: Map) -> Self {
        Battle {
            map,
            units: HashMap::new(),
            next_id: UnitId(0),
            turn_queue: VecDeque::new(),
            state: BattleState::Starting,
        }
    }

    pub fn active_unit(&self) -> Option<&Unit> {
        self.turn_queue.front().and_then(|id| self.units.get(id))
    }

    pub fn active_unit_mut(&mut self) -> Option<&mut Unit> {
        self.turn_queue.front().and_then(|id| self.units.get_mut(id))
    }

    pub fn unit_at(&self, coord: Coord) -> Option<&Unit> {
        self.units.values().find(|unit| unit.coord == coord)
    }

    pub fn add_unit<F>(&mut self, coord: Coord, f: F) -> UnitId
    where
        F: FnOnce(UnitId, Coord) -> Unit,
    {
        let id = self.next_id;
        let unit = f(id, coord);
        self.next_id.0 += 1;
        self.units.insert(id, unit);
        self.turn_queue.push_back(id);
        id
    }
}
