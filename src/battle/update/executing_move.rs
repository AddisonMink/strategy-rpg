use macroquad::prelude::trace;

use super::executing_effects;
use super::model::*;
use super::selecting_action;
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
        let unit = battle.active_unit().expect("No active unit.");
        match unit.side {
            Side::Player => {
                selecting_action::transition(battle);
            }
            Side::NPC => {
                let effects = (unit.select_action)(battle);
                executing_effects::transition(battle, effects);
            }
        }
    } else {
        timer.update(delta_time);
        if timer.is_finished() {
            *timer = Timer::new(MOVE_DURATION);
            let coord = path.pop_front().unwrap();
            let unit = battle.active_unit_mut().expect("No active unit.");
            let unit_id = unit.id;
            let side = unit.side;
            unit.coord = coord;
            battle.light_grid = LightGrid::new(battle);
            match side {
                Side::Player => {
                    update_all_npc_views_of_player(battle, unit_id);
                }
                Side::NPC => {
                    update_npc_view_of_all_players(battle, unit_id);
                }
            }
        }
    }
}

pub fn update_all_npc_views_of_player(battle: &mut Battle, player_id: UnitId) {
    let npc_ids: Vec<UnitId> = battle
        .unit_npc_iter()
        .map(|npc| npc.id)
        .filter(|npc_id| battle.unit_can_see_unit(*npc_id, player_id))
        .collect();

    for npc_id in npc_ids {
        update_last_seen(battle, npc_id, player_id);
    }
}

pub fn update_npc_view_of_all_players(battle: &mut Battle, npc_id: UnitId) {
    let visible_player_ids: Vec<UnitId> = battle
        .unit_player_iter()
        .map(|player| player.id)
        .filter(|player_id| battle.unit_can_see_unit(npc_id, *player_id))
        .collect();

    for player_id in visible_player_ids {
        update_last_seen(battle, npc_id, player_id);
    }
}

fn update_last_seen(battle: &mut Battle, npc_id: UnitId, player_id: UnitId) {
    let coord = battle.unit(player_id).unwrap().coord;
    let npc = battle.unit_mut(npc_id).expect("NPC unit not found.");

    if let Some((last_seen_id, last_seen_coord)) = npc.last_seen_player {
        let old_dist = last_seen_coord.manhattan_distance(npc.coord);
        let new_dist = coord.manhattan_distance(npc.coord);
        if last_seen_id == player_id || new_dist < old_dist {
            npc.last_seen_player = Some((player_id, coord));
        }
    } else {
        npc.last_seen_player = Some((player_id, coord));
    }
}

