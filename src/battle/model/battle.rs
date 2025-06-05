use macroquad::prelude::trace;

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

    pub fn unit(&self, id: UnitId) -> Option<&Unit> {
        self.units.get(&id)
    }

    pub fn unit_mut(&mut self, id: UnitId) -> Option<&mut Unit> {
        self.units.get_mut(&id)
    }

    pub fn active_unit(&self) -> Option<&Unit> {
        self.turn_queue.front().and_then(|id| self.units.get(id))
    }

    pub fn active_unit_mut(&mut self) -> Option<&mut Unit> {
        self.turn_queue
            .front()
            .and_then(|id| self.units.get_mut(id))
    }

    pub fn unit_at(&self, coord: Coord) -> Option<&Unit> {
        self.units.values().find(|unit| unit.coord == coord)
    }

    pub fn unit_iter(&self) -> impl Iterator<Item = &Unit> {
        self.units.values()
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

    pub fn next_turn(&mut self) {
        if self.turn_queue.len() > 1 {
            let id = self.turn_queue.pop_front().unwrap();
            self.turn_queue.push_back(id);
        }
    }
}
