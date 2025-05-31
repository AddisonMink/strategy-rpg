use super::GameState;
use super::LightGrid;
use super::Map;
use super::light::Light;
use super::point_light::*;
use super::unit::*;
use crate::util::*;
use std::collections::HashMap;
use std::collections::VecDeque;

pub struct Game {
    units: HashMap<UnitId, Unit>,
    next_unit_id: UnitId,
    point_lights: HashMap<PointLightId, PointLight>,
    next_point_light_id: PointLightId,
    turn_queue: VecDeque<UnitId>,
    pub map: Map,
    pub light_grid: LightGrid,
    pub state: GameState,
}

impl Game {
    pub fn new(map: Map) -> Self {
        Game {
            units: HashMap::new(),
            next_unit_id: UnitId(0),
            point_lights: HashMap::new(),
            next_point_light_id: PointLightId(0),
            turn_queue: VecDeque::new(),
            map,
            light_grid: LightGrid::empty(),
            state: GameState::Start,
        }
    }

    pub fn add_point_light(&mut self, f: impl FnOnce(PointLightId) -> PointLight) -> PointLightId {
        let id = self.next_point_light_id;
        let point_light = f(id);
        self.next_point_light_id.0 += 1;
        self.point_lights.insert(id, point_light);
        id
    }

    pub fn add_unit(&mut self, coord: Coord, f: impl FnOnce(UnitId, Coord) -> Unit) -> UnitId {
        let id = self.next_unit_id;
        let unit = f(id, coord);
        self.next_unit_id.0 += 1;
        self.units.insert(id, unit);
        id
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

    pub fn unit_players_iter(&self) -> impl Iterator<Item = &Unit> {
        self.units.values().filter(|unit| unit.is_player)
    }

    pub fn next_turn(&mut self) -> Option<UnitId> {
        self.turn_queue.pop_front();

        if self.turn_queue.is_empty() {
            for unit_id in self.units.keys() {
                self.turn_queue.push_back(*unit_id);
            }
        }

        self.turn_queue.front().cloned()
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

    pub fn any_player_can_see(&self, coord: Coord) -> bool {
        self.unit_players_iter()
            .any(|unit| self.player_can_see(unit.id, coord))
    }

    pub fn player_can_see(&self, player_id: UnitId, coord: Coord) -> bool {
        if let Some(player) = self.unit(player_id) {
            let distance = player.coord.manhattan_distance(coord);
            let distance_from_light = self.light_grid.distance_from_light(coord);
            self.map.check_line_of_sight(player.coord, coord)
                && (distance <= player.vision || distance_from_light <= player.vision)
        } else {
            false
        }
    }
}
