use super::model::*;
use super::selecting_move;
use crate::engine::*;
use std::collections::VecDeque;

const MOVE_DURATION: f32 = 0.2;

pub fn transition(battle: &mut Battle, path: VecDeque<Coord>) {
    let timer = Timer::new(0.0);
    battle.state = BattleState::ExecutingMove { path, timer };
}

pub fn update(battle: &mut Battle, delta_time: f32) {
    let BattleState::ExecutingMove { path, timer } = &mut battle.state else {
        return;
    };

    if path.is_empty() {
        selecting_move::transition(battle);
    } else {
        timer.update(delta_time);
        if timer.is_finished() {
            *timer = Timer::new(MOVE_DURATION);
            let coord = path.pop_front().unwrap();
            let unit = battle.active_unit_mut().expect("No active unit.");
            unit.coord = coord;
        }
    }
}
