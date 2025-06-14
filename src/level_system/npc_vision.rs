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

pub fn update_all_npc_vision_of_player(level: &mut Level, player: Entity) {
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

pub fn update_npc_vision_of_all_players(level: &mut Level, npc: Entity) {
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

fn update_npc_vision_of_player(level: &mut Level, npc: Entity, player: Entity) -> Option<()> {
    let player_coord = level.positions.get(&player).map(|p| p.coord)?;
    let coord = level.positions.get(&npc).map(|p| p.coord)?;
    let visible = level.unit_can_see_unit(npc, player);
    let memory = level.vision_memory.get_mut(&npc)?;
    let player_seen = visible && !memory.visible_players.contains(&player);
    let player_lost = !visible && memory.visible_players.contains(&player);

    if visible {
        memory.last_seen_player = Some((player, player_coord));
    }

    if player_seen {
        memory.visible_players.insert(player);
        level
            .animation_queue
            .push_back(Animation::text(coord, "!".to_string(), RED));
    } else if player_lost {
        memory.visible_players.remove(&player);
        level
            .animation_queue
            .push_back(Animation::text(coord, "?".to_string(), YELLOW));
    }

    Some(())
}
