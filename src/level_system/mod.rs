mod action;
mod effect;
mod light_grid;
mod npc_vision;
mod player_vision;
mod state;

use crate::level_model::*;
use effect::process_effects;

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
        LevelState::Success => UpdateResult::Stop,
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
