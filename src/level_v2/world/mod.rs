mod effect;
mod light;
mod light_grid;
mod map;
mod point_light;
mod tile;
mod unit;

use crate::engine_v2::*;
use std::collections::{HashMap, VecDeque};

pub use effect::*;
pub use light::*;
pub use light_grid::*;
pub use map::*;
pub use point_light::*;
pub use tile::*;
pub use unit::*;

use crate::util::Coord;

pub struct World {
    // Resources
    pub map: Map,
    pub light_grid: LightGrid,
    // Units
    units: HashMap<UnitId, Unit>,
    next_unit_id: u32,
    unit_queue: VecDeque<UnitId>,
    // Lights
    point_lights: HashMap<PointLightId, PointLight>,
    next_point_light_id: u32,
    // State
    pub effects: VecDeque<Effect>,
}

impl World {
    pub fn new() -> Self {
        Self {
            map: Map::new(),
            light_grid: LightGrid::empty(),
            units: HashMap::new(),
            next_unit_id: 0,
            unit_queue: VecDeque::new(),
            point_lights: HashMap::new(),
            next_point_light_id: 0,
            effects: VecDeque::new(),
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

    pub fn remove_unit(&mut self, id: UnitId) -> Option<Unit> {
        self.unit_queue.retain(|&unit_id| unit_id != id);
        self.units.remove(&id)
    }

    pub fn unit(&self, id: UnitId) -> Option<&Unit> {
        self.units.get(&id)
    }

    pub fn unit_at(&self, coord: Coord) -> Option<&Unit> {
        self.units.values().find(|unit| unit.coord == coord)
    }

    pub fn active_unit(&self) -> Option<&Unit> {
        self.unit_queue.front().and_then(|id| self.units.get(id))
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
        self.point_lights.values().map(|l| (l.coord, &l.light))
    }
}
