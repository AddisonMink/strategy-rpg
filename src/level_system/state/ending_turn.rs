use super::*;

pub fn transition(level: &mut Level) {
    level.state = LevelState::EndingTurn;
}

pub fn update(level: &mut Level) {
    if let Some(entity) = level.turn_queue.pop_front() {
        level.turn_queue.push_back(entity);
    }

    if check_goal(level) {
        level.state = LevelState::Success;
    } else {
        selecting_move::transition(level);
    }
}

pub fn check_goal(level: &Level) -> bool {
    level
        .units
        .values()
        .filter(|unit| unit.side == Side::Player)
        .map(|u| u.coord)
        .any(|c| level.map.tile(c).goal)
}
