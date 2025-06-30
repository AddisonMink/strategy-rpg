mod action;
mod animation;
mod effect;
mod goal;
mod item;
mod light;
mod light_grid;
mod map;
mod player_vision;
mod point_light;
mod tile;
mod unit;

use crate::engine::*;
use crate::util::*;
use std::collections::{HashMap, VecDeque};

pub use action::*;
pub use animation::*;
pub use effect::*;
pub use goal::*;
pub use item::*;
pub use light::*;
pub use light_grid::*;
use macroquad::prelude::trace;
pub use map::*;
pub use player_vision::*;
pub use point_light::*;
pub use tile::*;
pub use unit::*;

use crate::util::Coord;

pub struct World {
    // Resources
    pub map: Map,
    pub light_grid: LightGrid,
    pub player_vision: PlayerVision,
    // Units
    units: HashMap<UnitId, Unit>,
    next_unit_id: u32,
    unit_queue: VecDeque<UnitId>,
    // Lights
    point_lights: HashMap<PointLightId, PointLight>,
    next_point_light_id: u32,
    // State
    pub effects: VecDeque<Effect>,
    pub animations: VecDeque<Animation>,
    pub goal: Goal,
}

impl World {
    pub fn new() -> Self {
        Self {
            map: Map::new(),
            light_grid: LightGrid::empty(),
            player_vision: PlayerVision::empty(),
            units: HashMap::new(),
            next_unit_id: 0,
            unit_queue: VecDeque::new(),
            point_lights: HashMap::new(),
            next_point_light_id: 0,
            effects: VecDeque::new(),
            animations: VecDeque::new(),
            goal: Goal::KillAllEnemies,
        }
    }

    pub fn add_unit(&mut self, data: UnitData, coord: Coord) -> UnitId {
        let id = UnitId(self.next_unit_id);
        let unit = Unit::new(id, data, coord);

        self.units.insert(id, unit);
        self.next_unit_id += 1;
        self.unit_queue.push_back(id);

        id
    }

    pub fn add_unit_with_items(
        &mut self,
        data: UnitData,
        coord: Coord,
        items: HashMap<ItemId, Item>,
    ) -> UnitId {
        let id = self.add_unit(data, coord);
        let unit = self.units.get_mut(&id).expect("Unit should exist");
        unit.items = items;
        id
    }

    pub fn remove_unit(&mut self, id: UnitId) -> Option<Unit> {
        self.unit_queue.retain(|&unit_id| unit_id != id);
        self.units.remove(&id)
    }

    pub fn unit(&self, id: UnitId) -> Option<&Unit> {
        self.units.get(&id)
    }

    pub fn unit_mut(&mut self, id: UnitId) -> Option<&mut Unit> {
        self.units.get_mut(&id)
    }

    pub fn unit_at(&self, coord: Coord) -> Option<&Unit> {
        self.units.values().find(|unit| unit.coord == coord)
    }

    pub fn active_unit_id(&self) -> Option<UnitId> {
        self.unit_queue.front().cloned()
    }

    pub fn active_unit(&self) -> Option<&Unit> {
        self.unit_queue.front().and_then(|id| self.units.get(id))
    }

    pub fn active_unit_mut(&mut self) -> Option<&mut Unit> {
        self.unit_queue
            .front()
            .and_then(|id| self.units.get_mut(id))
    }

    pub fn units_iter(&self) -> impl Iterator<Item = &Unit> {
        self.units.values()
    }

    pub fn player_units_iter(&self) -> impl Iterator<Item = &Unit> {
        self.units
            .values()
            .filter(|unit| unit.data().side == Side::Player)
    }

    pub fn npc_units_iter(&self) -> impl Iterator<Item = &Unit> {
        self.units
            .values()
            .filter(|unit| unit.data().side == Side::NPC)
    }

    pub fn end_turn(&mut self) -> Option<UnitId> {
        if let Some(id) = self.unit_queue.pop_front() {
            self.unit_queue.push_back(id);
            self.unit_queue.front().cloned()
        } else {
            None
        }
    }

    pub fn add_point_light(&mut self, radius: u16, color: Color, coord: Coord) -> PointLightId {
        let id = PointLightId(self.next_point_light_id);
        let light = PointLight::new(id, radius, color, coord);

        self.point_lights.insert(id, light);
        self.next_point_light_id += 1;

        id
    }

    pub fn remove_point_light(&mut self, id: PointLightId) -> Option<PointLight> {
        self.point_lights.remove(&id)
    }

    pub fn lights_iter(&self) -> impl Iterator<Item = (Coord, &Light)> {
        let point_lights = self.point_lights.values().map(|l| (l.coord, &l.light));

        let unit_lights = self
            .units
            .values()
            .filter_map(|unit| unit.light.as_ref().map(|light| (unit.coord, light)));

        point_lights.chain(unit_lights)
    }

    pub fn unit_can_see_tile(&self, unit_id: UnitId, coord: Coord) -> bool {
        let Some(unit) = self.unit(unit_id) else {
            return false;
        };

        if !self.map.check_line_of_sight(unit.coord, coord) {
            return false;
        }

        let distance = unit.coord.manhattan_distance(coord);
        let distance_from_light = self.light_grid.distance_from_light(coord);
        distance <= unit.data().vision || distance_from_light <= unit.data().vision
    }

    pub fn unit_can_see_unit(&self, observer_id: UnitId, target_id: UnitId) -> bool {
        let Some(target) = self.unit(target_id) else {
            return false;
        };

        if target.tags.contains(&UnitTag::Lurker) {
            let distance_from_light = self.light_grid.distance_from_light(target.coord);
            if distance_from_light > 0 {
                return false;
            }
        }

        self.unit_can_see_tile(observer_id, target.coord)
    }

    pub fn valid_move(&self, coord: Coord) -> bool {
        self.map.tile(coord).walkable && self.unit_at(coord).is_none()
    }
}
