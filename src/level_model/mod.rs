mod action;
mod animation;
mod effect;
mod goal;
mod item;
mod level_state;
mod light;
mod light_grid;
mod map;
mod player_vision;
mod point_light;
mod unit;

use crate::engine::*;
use std::collections::{HashMap, VecDeque};

pub use action::*;
pub use animation::*;
pub use effect::*;
pub use goal::*;
pub use item::*;
pub use level_state::*;
pub use light::*;
pub use light_grid::*;
pub use map::*;
pub use player_vision::*;
pub use point_light::*;
pub use unit::*;

pub struct Level {
    // Resources
    pub map: Map,
    pub light_grid: LightGrid,
    pub player_vision: PlayerVision,
    // Entities
    pub point_lights: HashMap<PointLightId, PointLight>,
    pub next_point_light_id: PointLightId,
    pub units: HashMap<UnitId, Unit>,
    pub next_unit_id: UnitId,
    // State
    pub state: LevelState,
    pub turn_queue: VecDeque<UnitId>,
    pub effect_queue: VecDeque<Effect>,
    pub sleep_timer: Option<Timer>,
    pub animation_queue: VecDeque<Animation>,
    pub goal: Goal,
}

impl Level {
    pub fn empty() -> Self {
        Self {
            map: Map::new(),
            light_grid: LightGrid::empty(),
            player_vision: PlayerVision::empty(),
            point_lights: HashMap::new(),
            next_point_light_id: PointLightId(0),
            units: HashMap::new(),
            next_unit_id: UnitId(0),
            state: LevelState::Initializing,
            turn_queue: VecDeque::new(),
            effect_queue: VecDeque::new(),
            sleep_timer: None,
            animation_queue: VecDeque::new(),
            goal: Goal::ReachGoalTile,
        }
    }

    pub fn delete_unit(&mut self, entity: UnitId) {
        self.units.remove(&entity);
        self.turn_queue.retain(|id| *id != entity);
        self.effect_queue.retain(|effect| match effect {
            Effect::Move { entity: e, .. } => *e != entity,
            _ => true,
        });
    }

    pub fn lights_iter(&self) -> impl Iterator<Item = (Coord, Light)> {
        let point_lights = self
            .point_lights
            .values()
            .map(|l| (l.coord, l.light.clone()));

        let unit_lights = self
            .units
            .values()
            .filter_map(|u| u.light.map(|l| (u.coord, l)));

        point_lights.chain(unit_lights)
    }

    pub fn active_unit(&self) -> Option<&Unit> {
        let active_entity = self.turn_queue.front()?;
        self.units.get(active_entity)
    }

    pub fn unit_at(&self, coord: Coord) -> Option<&Unit> {
        self.units.values().find(|unit| unit.coord == coord)
    }

    pub fn unit_can_see_tile(&self, unit_id: UnitId, coord: Coord) -> bool {
        let Some(unit) = self.units.get(&unit_id) else {
            return false;
        };

        if !self.map.check_line_of_sight(unit.coord, coord) {
            return false;
        }

        let distance = unit.coord.manhattan_distance(coord);
        let distance_from_light = self.light_grid.distance_from_light(coord);
        distance <= unit.vision || distance_from_light <= unit.vision
    }

    pub fn unit_can_see_unit(&self, from: UnitId, to: UnitId) -> bool {
        let Some(from_unit) = self.units.get(&from) else {
            return false;
        };

        let Some(to_unit) = self.units.get(&to) else {
            return false;
        };

        let lurker = to_unit.tags.contains(&UnitTag::Lurker);

        if !self.map.check_line_of_sight(from_unit.coord, to_unit.coord) {
            return false;
        }

        let distance = from_unit.coord.manhattan_distance(to_unit.coord);
        let distance_from_light = self.light_grid.distance_from_light(to_unit.coord);

        if distance_from_light > 0 && lurker {
            return false; // Lurkers are hidden when not in light.
        }
        distance <= from_unit.vision || distance_from_light <= from_unit.vision
    }
}
