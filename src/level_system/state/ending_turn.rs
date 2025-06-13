use super::*;
use crate::engine::*;

pub fn transition(level: &mut Level) {
    level.state = LevelState::EndingTurn;
}

pub fn update(level: &mut Level) {
    if let Some(entity) = level.turn_queue.pop_front() {
        level.turn_queue.push_back(entity);
    }
    selecting_move::transition(level);
}
