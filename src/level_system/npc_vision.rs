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

    for player in players.iter() {
        update_npc_vision_of_player(level, *player);
    }
}

pub fn update_npc_vision_of_player(level: &mut Level, player: Entity) {
    let Some(player_coord) = level.positions.get(&player).map(|p| p.coord) else {
        return;
    };

    let npcs = level
        .units
        .iter()
        .filter(|(_, u)| u.side == Side::NPC)
        .map(|(e, u)| *e)
        .collect::<Vec<_>>();

    for entity in npcs.iter() {
        let visible = level.unit_can_see_tile(*entity, player_coord);

        let Some(coord) = level.positions.get(entity).map(|p| p.coord) else {
            continue;
        };

        let Some(memory) = level.vision_memory.get_mut(entity) else {
            continue;
        };

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
    }
}
