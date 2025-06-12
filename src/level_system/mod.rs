mod action;
mod light_grid;
mod player_vision;
mod selecting_action;
mod selecting_move;
mod selecting_single_unit_target;
mod selecting_target;

use crate::engine::*;
use crate::level_model::*;
use light_grid::update_light_grid;
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

    process_effects(level);
    if !level.effect_queue.is_empty() {
        return UpdateResult::Stop;
    }

    process_state(level);
    if level.effect_queue.is_empty() {
        UpdateResult::Stop
    } else {
        UpdateResult::Continue
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

fn process_effects(level: &mut Level) {
    while let Some(effect) = level.effect_queue.pop_front() {
        match effect {
            Effect::UpdateLightGrid => update_light_grid(level),
            Effect::UpdateVisionGrid => update_player_vision(level),
            Effect::Move { entity, coord } => execute_move(level, entity, coord),
            Effect::Sleep { duration } => level.sleep_timer = Some(Timer::new(duration)),
            _ => {}
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

fn process_state(level: &mut Level) {
    match level.state {
        LevelState::Starting => {
            level.effect_queue.push_back(Effect::UpdateLightGrid);
            level.effect_queue.push_back(Effect::UpdateVisionGrid);
            selecting_move::transition(level);
        }
        LevelState::SelectingMove { .. } => selecting_move::update(level),
        LevelState::ResolvingMove => selecting_action::transition(level),
        LevelState::SelectingAction { .. } => selecting_action::update(level),
        LevelState::SelectingSingleUnitTarget { .. } => selecting_single_unit_target::update(level),
        _ => {}
    }
}
