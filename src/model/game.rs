use super::LightGrid;
use super::Map;
use super::light::Light;
use super::point_light::*;
use super::unit::*;
use crate::util::*;
use std::collections::HashMap;

pub struct Game {
    units: HashMap<UnitId, Unit>,
    next_unit_id: UnitId,
    point_lights: HashMap<PointLightId, PointLight>,
    next_point_light_id: PointLightId,
    pub map: Map,
    pub light_grid: LightGrid,
}

impl Game {
    pub fn new(map: Map) -> Self {
        Game {
            units: HashMap::new(),
            next_unit_id: UnitId(0),
            point_lights: HashMap::new(),
            next_point_light_id: PointLightId(0),
            map,
            light_grid: LightGrid::empty(),
        }
    }

    pub fn add_point_light(&mut self, f: impl FnOnce(PointLightId) -> PointLight) -> PointLightId {
        let id = self.next_point_light_id;
        let point_light = f(id);
        self.next_point_light_id.0 += 1;
        self.point_lights.insert(id, point_light);
        id
    }

    pub fn add_unit(&mut self, f: impl FnOnce(UnitId) -> Unit) -> UnitId {
        let id = self.next_unit_id;
        let unit = f(id);
        self.next_unit_id.0 += 1;
        self.units.insert(id, unit);
        id
    }

    pub fn unit(&self, id: UnitId) -> Option<&Unit> {
        self.units.get(&id)
    }

    pub fn unit_at(&self, coord: Coord) -> Option<&Unit> {
        self.units.values().find(|unit| unit.coord == coord)
    }

    pub fn unit_mut(&mut self, id: UnitId) -> Option<&mut Unit> {
        self.units.get_mut(&id)
    }

    pub fn lights_iter(&self) -> impl Iterator<Item = (Coord, &Light)> {
        let point_lights_iter = self
            .point_lights
            .iter()
            .map(|(_, pl)| (pl.coord, &pl.light));

        let unit_lights_iter = self
            .units
            .iter()
            .filter_map(|(_, unit)| unit.light.as_ref().map(|light| (unit.coord, light)));

        point_lights_iter.chain(unit_lights_iter)
    }
}
