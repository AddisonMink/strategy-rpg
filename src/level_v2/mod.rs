mod content;
mod draw;
mod state;
mod update;
mod world;

use crate::engine_v2::*;
use crate::util::*;
use state::*;
use world::*;

pub use state::LevelResult;

pub struct Level {
    world: World,
    state: State,
}

impl Level {
    pub fn new() -> Self {
        let hero_data = UnitData {
            name: ShortString::new("Hero"),
            glyph: Glyph::new('@', WHITE),
            side: Side::Player,
            vision: 2,
            movement: 3,
            hp_max: 5,
            behavior: None,
        };

        let goon_data = content::unit::GOON_DATA;

        let mut world = World::new();
        world.add_unit(hero_data, Coord::new(1, 1));
        world.add_unit(goon_data, Coord::new(5, 1));
        world.add_point_light(3, ORANGE, Coord::new(1, 1));

        Self {
            world,
            state: State::Starting,
        }
    }

    pub fn update(&mut self, delta_time: f32) -> Option<LevelResult> {
        update::update(&mut self.world, &mut self.state, delta_time)
    }

    pub fn draw(&self) {
        draw::draw(&self.world, &self.state);
    }
}
