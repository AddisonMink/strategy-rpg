use crate::engine::*;
use crate::level_content::*;
use crate::level_model::*;
use crate::level_render::render_level;
use crate::level_system::*;

pub struct Game {
    pub level: Level,
    pub state: GameState,
}

impl Game {
    pub fn new() -> Self {
        let level = new_level();
        Self {
            level,
            state: GameState::Playing,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Playing,
    Complete,
}

fn new_level() -> Level {
    let mut level = Level::empty();
    add_hero(&mut level, Coord::new(1, 1));
    add_goon(&mut level, Coord::new(5, 1));
    add_shadow(&mut level, Coord::new(8, 4));
    add_point_light(&mut level, Coord::new(1, 1), 3, BLUE);
    level
}

pub fn update_game(game: &mut Game, delta_time: f32) {
    match game.state {
        GameState::Playing => update_playing(game, delta_time),
        GameState::Complete => update_complete(game),
    }
}

fn update_playing(game: &mut Game, delta_time: f32) {
    let mut result = update_level(&mut game.level, delta_time);

    while result == UpdateResult::Continue {
        result = update_level(&mut game.level, delta_time);
    }

    if result == UpdateResult::LevelComplete {
        game.state = GameState::Complete;
    }
}

fn update_complete(game: &mut Game) {
    if input::mouse_clicked() {
        game.level = new_level();
        game.state = GameState::Playing;
    }
}

pub fn render_game(game: &Game) {
    render_level(&game.level);
    if game.state == GameState::Complete {
        grid::draw_sub_message("Click to restart.", WHITE);
    }
}
