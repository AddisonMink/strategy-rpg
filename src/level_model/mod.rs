mod action;
mod animation;
mod effect;
mod entity;
mod item;
mod level_state;
mod light_grid;
mod map;
mod player_vision;

use crate::engine::*;
use std::collections::{HashMap, VecDeque};

pub use item::*;
pub use action::*;
pub use animation::*;
pub use effect::*;
pub use entity::*;
pub use level_state::*;
pub use light_grid::*;
use macroquad::prelude::trace;
pub use map::*;
pub use player_vision::*;

pub struct Level {
    // Resources
    pub map: Map,
    pub light_grid: LightGrid,
    pub player_vision: PlayerVision,
    // Entities
    pub positions: HashMap<Entity, Position>,
    pub tags: HashMap<Entity, Tags>,
    pub vision_memory: HashMap<Entity, VisionMemory>,
    pub lights: HashMap<Entity, Light>,
    pub units: HashMap<Entity, Unit>,
    pub behaviors: HashMap<Entity, Behavior>,
    pub inventories: HashMap<Entity, Inventory>,
    pub next_id: Entity,
    // State
    pub state: LevelState,
    pub turn_queue: VecDeque<Entity>,
    pub effect_queue: VecDeque<Effect>,
    pub sleep_timer: Option<Timer>,
    pub animation_queue: VecDeque<Animation>,
}

impl Level {
    pub fn empty() -> Self {
        Self {
            map: Map::new(),
            light_grid: LightGrid::empty(),
            player_vision: PlayerVision::empty(),
            positions: HashMap::new(),
            tags: HashMap::new(),
            vision_memory: HashMap::new(),
            lights: HashMap::new(),
            units: HashMap::new(),
            behaviors: HashMap::new(),
            inventories: HashMap::new(),
            next_id: Entity(0),
            state: LevelState::Starting,
            turn_queue: VecDeque::new(),
            effect_queue: VecDeque::new(),
            sleep_timer: None,
            animation_queue: VecDeque::new(),
        }
    }

    pub fn delete(&mut self, entity: Entity) {
        self.positions.remove(&entity);
        self.tags.remove(&entity);
        self.vision_memory.remove(&entity);
        self.lights.remove(&entity);
        self.units.remove(&entity);
        self.turn_queue.retain(|id| *id != entity);
        self.effect_queue.retain(|effect| match effect {
            Effect::Move { entity: e, .. } => *e != entity,
            _ => true,
        });
    }

    pub fn active_unit(&self) -> Option<&Unit> {
        self.turn_queue.front().and_then(|id| self.units.get(id))
    }

    pub fn active_unit_with_position(&self) -> Option<(&Position, &Unit)> {
        self.active_unit()
            .and_then(|unit| self.positions.get(&unit.entity).map(|pos| (pos, unit)))
    }

    pub fn unit_at(&self, coord: Coord) -> Option<&Unit> {
        self.positions
            .values()
            .filter_map(|p| {
                if p.coord == coord {
                    self.units.get(&p.entity)
                } else {
                    None
                }
            })
            .next()
    }

    pub fn unit_can_see_tile(&self, unit_id: Entity, coord: Coord) -> bool {
        let Some(pos) = self.positions.get(&unit_id) else {
            return false;
        };

        let Some(unit) = self.units.get(&unit_id) else {
            return false;
        };

        if !self.map.check_line_of_sight(pos.coord, coord) {
            return false;
        }

        let distance = pos.coord.manhattan_distance(coord);
        let distance_from_light = self.light_grid.distance_from_light(coord);
        distance <= unit.vision || distance_from_light <= unit.vision
    }

    pub fn unit_can_see_unit(&self, from: Entity, to: Entity) -> bool {
        let Some(from_pos) = self.positions.get(&from) else {
            return false;
        };
        let Some(from_unit) = self.units.get(&from) else {
            return false;
        };
        let Some(to_pos) = self.positions.get(&to) else {
            return false;
        };

        let lurker = if let Some(tags) = self.tags.get(&to) {
            tags.tags.contains(&EntityTag::Lurker)
        } else {
            false
        };

        if !self.map.check_line_of_sight(from_pos.coord, to_pos.coord) {
            return false;
        }

        let distance = from_pos.coord.manhattan_distance(to_pos.coord);
        let distance_from_light = self.light_grid.distance_from_light(to_pos.coord);

        if distance_from_light > 0 && lurker {
            return false; // Lurkers are hidden when not in light.
        }
        distance <= from_unit.vision || distance_from_light <= from_unit.vision
    }
}
