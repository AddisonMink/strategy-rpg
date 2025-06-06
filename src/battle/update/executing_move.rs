use super::executing_effects;
use super::model::*;
use super::selecting_action;
use crate::engine::*;
use std::collections::VecDeque;

const MOVE_DURATION: f32 = 0.1;

pub fn transition(battle: &mut Battle, path: VecDeque<Coord>) {
    let timer = Timer::new(0.0);
    battle.state = BattleState::ExecutingMove { path, timer };
}

pub fn update(battle: &mut Battle, delta_time: f32) {
    let BattleState::ExecutingMove { path, timer } = &mut battle.state else {
        return;
    };

    if path.is_empty() {
        let unit = battle.active_unit().expect("No active unit.");
        match unit.side {
            Side::Player => selecting_action::transition(battle),
            Side::NPC => {
                let effects = unit.npc_select_action(battle).unwrap_or_default();
                executing_effects::transition(battle, effects);
            }
        }
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
