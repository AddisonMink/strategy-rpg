use macroquad::prelude::trace;

use super::*;
use crate::engine::*;

pub fn update_all_npc_vision(level: &mut Level) {
    let players = level
        .units
        .iter()
        .filter(|(_, u)| u.side == Side::Player)
        .map(|(e, _)| *e)
        .collect::<Vec<_>>();

    let npcs = level
        .units
        .iter()
        .filter(|(_, u)| u.side == Side::NPC)
        .map(|(e, _)| *e)
        .collect::<Vec<_>>();

    for player in players.iter() {
        for npc in npcs.iter() {
            update_npc_vision_of_player(level, *npc, *player);
        }
    }
}

pub fn update_all_npc_vision_of_player(level: &mut Level, player: UnitId) {
    let npcs = level
        .units
        .iter()
        .filter(|(_, u)| u.side == Side::NPC)
        .map(|(e, _)| *e)
        .collect::<Vec<_>>();

    for npc in npcs.iter() {
        update_npc_vision_of_player(level, *npc, player);
    }
}

pub fn update_npc_vision_of_all_players(level: &mut Level, npc: UnitId) {
    let players = level
        .units
        .iter()
        .filter(|(_, u)| u.side == Side::Player)
        .map(|(e, _)| *e)
        .collect::<Vec<_>>();

    for player in players.iter() {
        update_npc_vision_of_player(level, npc, *player);
    }
}

fn update_npc_vision_of_player(level: &mut Level, npc_id: UnitId, player_id: UnitId) -> Option<()> {
    let visible = level.unit_can_see_unit(npc_id, player_id);
    let player_coord = level.units.get(&player_id)?.coord;
    let coord = level.units.get(&npc_id)?.coord;
    let npc = level.units.get_mut(&npc_id)?;
    let player_seen = visible && !npc.memory.visible_players.contains(&player_id);
    let player_lost = !visible && npc.memory.visible_players.contains(&player_id);

    if visible {
        npc.memory.last_seen_player = Some((player_id, player_coord));
    }

    if player_seen {
        npc.memory.visible_players.insert(player_id);
        level
            .animation_queue
            .push_back(Animation::text(coord, "!".to_string(), RED));
    } else if player_lost {
        npc.memory.visible_players.remove(&player_id);
        level
            .animation_queue
            .push_back(Animation::text(coord, "?".to_string(), YELLOW));
    }

    Some(())
}
