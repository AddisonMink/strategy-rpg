mod action;
mod light_grid;
mod player_vision;
mod state;

use crate::engine::*;
use crate::level_model::*;
use light_grid::update_light_grid;
use macroquad::text;
use player_vision::update_player_vision;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateResult {
    Continue,
    Stop,
}

pub fn update_level(level: &mut Level, delta_time: f32) -> UpdateResult {
    process_timer(level, delta_time);
    if level.sleep_timer.is_some() {
        return UpdateResult::Stop;
    }

    process_animations(level, delta_time);
    if !level.animation_queue.is_empty() {
        return UpdateResult::Stop;
    }

    process_effects(level);
    if !level.animation_queue.is_empty() || level.sleep_timer.is_some() {
        return UpdateResult::Stop;
    }

    state::process_state(level);
    match &level.state {
        LevelState::SelectingMove { .. } => UpdateResult::Stop,
        LevelState::SelectingAction { .. } => UpdateResult::Stop,
        LevelState::SelectingSingleUnitTarget { .. } => UpdateResult::Stop,
        _ => UpdateResult::Continue,
    }
}

fn process_timer(level: &mut Level, delta_time: f32) {
    let Some(sleep_timer) = level.sleep_timer.as_mut() else {
        return;
    };
    sleep_timer.update(delta_time);
    if sleep_timer.is_finished() {
        level.sleep_timer = None;
    }
}

fn process_animations(level: &mut Level, delta_time: f32) {
    while let Some(animation) = level.animation_queue.front_mut() {
        animation.timer.update(delta_time);
        if animation.timer.is_finished() {
            level.animation_queue.pop_front();
        } else {
            break;
        }
    }
}

fn process_effects(level: &mut Level) {
    while let Some(effect) = level.effect_queue.pop_front() {
        match effect {
            Effect::UpdateLightGrid => update_light_grid(level),
            Effect::UpdateVisionGrid => update_player_vision(level),
            Effect::Move { entity, coord } => execute_move(level, entity, coord),
            Effect::Sleep { duration } => level.sleep_timer = Some(Timer::new(duration)),
            Effect::Damage { entity, min, max } => execute_damage(level, entity, min, max),
        }
        if level.sleep_timer.is_some() {
            break;
        }
    }
}

fn execute_move(level: &mut Level, entity: Entity, coord: Coord) {
    let Some(pos) = level.positions.get_mut(&entity) else {
        return;
    };

    let is_player = level
        .units
        .get(&entity)
        .map_or(false, |unit| unit.side == Side::Player);

    pos.coord = coord;
    if is_player {
        level.effect_queue.push_front(Effect::UpdateVisionGrid);
    }
}

fn execute_damage(level: &mut Level, entity: Entity, min: u16, max: u16) {
    let Some(unit) = level.units.get_mut(&entity) else {
        return;
    };

    let coord = level.positions.get(&entity).unwrap().coord;
    let damage = max;
    let text = (-(damage as i32)).to_string();

    unit.hp = unit.hp.saturating_sub(damage);
    level
        .animation_queue
        .push_back(Animation::text(coord, text, RED));
}
