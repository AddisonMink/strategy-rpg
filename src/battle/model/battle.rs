use super::Map;
use super::*;
use crate::engine::*;
use std::collections::{HashMap, VecDeque};

pub struct Battle {
    pub map: Map,
    units: HashMap<UnitId, Unit>,
    next_unit_id: UnitId,
    turn_queue: VecDeque<UnitId>,
    point_lights: HashMap<Coord, Light>,
    pub light_grid: LightGrid,
    pub state: BattleState,
}

impl Battle {
    pub fn new(map: Map) -> Self {
        Battle {
            map,
            units: HashMap::new(),
            next_unit_id: UnitId(0),
            turn_queue: VecDeque::new(),
            point_lights: HashMap::new(),
            light_grid: LightGrid::empty(),
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

    pub fn unit_can_see_tile(&self, from: UnitId, to: Coord) -> bool {
        let Some(from_unit) = self.unit(from) else {
            return false;
        };
        let distance = from_unit.coord.manhattan_distance(to);
        let distance_from_light = self.light_grid.distance_from_light(to);
        self.map.check_line_of_sight(from_unit.coord, to)
            && (distance <= from_unit.vision || distance_from_light <= from_unit.vision)
    }

    pub fn unit_can_see_unit(&self, from: UnitId, to: UnitId) -> bool {
        let Some(from_unit) = self.unit(from) else {
            return false;
        };
        let Some(to_unit) = self.unit(to) else {
            return false;
        };
        let distance = from_unit.coord.manhattan_distance(to_unit.coord);
        let distance_from_light = self.light_grid.distance_from_light(to_unit.coord);

        if distance_from_light > 1 && to_unit.tags.contains(&UnitTag::Lurker) {
            return false; // Lurkers are hidden when not in light.
        } else {
            self.map.check_line_of_sight(from_unit.coord, to_unit.coord)
                && (distance <= from_unit.vision || distance_from_light <= from_unit.vision)
        }
    }

    pub fn unit_iter(&self) -> impl Iterator<Item = &Unit> {
        self.units.values()
    }

    pub fn unit_player_iter(&self) -> impl Iterator<Item = &Unit> {
        self.units.values().filter(|unit| unit.side == Side::Player)
    }

    pub fn unit_npc_iter(&self) -> impl Iterator<Item = &Unit> {
        self.units.values().filter(|unit| unit.side == Side::NPC)
    }

    pub fn add_unit<F>(&mut self, coord: Coord, f: F) -> UnitId
    where
        F: FnOnce(UnitId, Coord) -> Unit,
    {
        let id = self.next_unit_id;
        let unit = f(id, coord);
        self.next_unit_id.0 += 1;
        self.units.insert(id, unit);
        self.turn_queue.push_back(id);
        id
    }

    pub fn remove_unit(&mut self, id: UnitId) {
        self.units.remove(&id);
        self.turn_queue.retain(|&x| x != id);
    }

    pub fn lights_iter(&self) -> impl Iterator<Item = (&Coord, &Light)> {
        self.point_lights.iter()
    }

    pub fn add_light(&mut self, coord: Coord, light: Light) {
        self.point_lights.insert(coord, light);
        self.light_grid = LightGrid::new(&self);
    }

    pub fn get_light_grid(&self) -> &LightGrid {
        &self.light_grid
    }

    pub fn next_turn(&mut self) {
        if self.turn_queue.len() > 1 {
            let id = self.turn_queue.pop_front().unwrap();
            self.turn_queue.push_back(id);
        }
    }
}
