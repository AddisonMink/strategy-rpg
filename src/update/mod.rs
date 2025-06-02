use crate::prelude::*;
use ending_turn::*;
use executing_effects::*;
use executing_move::*;
use npc_executing_move::*;
use npc_selecting_action::*;
use npc_selecting_move::*;
use selecting_action::*;
use selecting_move::*;
use selecting_single_unit_target::*;
use showing_animations::*;
use starting_turn::*;
use std::collections::VecDeque;

mod ending_turn;
mod executing_effects;
mod executing_move;
mod npc_executing_move;
mod npc_selecting_action;
mod npc_selecting_move;
mod selecting_action;
mod selecting_move;
mod selecting_single_unit_target;
mod showing_animations;
mod starting_turn;

pub fn update_game(game: &mut Game, delta_time: f32) -> Option<()> {
    match game.state.clone() {
        GameState::Start => to_starting_turn(game),
        GameState::StartingTurn { .. } => update_starting_turn(game, delta_time),
        GameState::SelectingMove { .. } => update_selecting_move(game),
        GameState::NpcSelectingMove => update_npc_selecting_move(game),
        GameState::ExecutingMove { .. } => update_executing_move(game),
        GameState::NpcExecutingMove { .. } => update_npc_executing_move(game, delta_time),
        GameState::SelectingAction { .. } => update_selecting_action(game),
        GameState::SelectingSingleUnitTarget { .. } => update_selecting_single_unit_target(game),
        GameState::NpcSelectingAction => update_npc_selecting_action(game),
        GameState::ExecutingEffects { .. } => update_executing_effects(game),
        GameState::ShowingAnimations { .. } => update_showing_animations(game, delta_time),
        GameState::EndingTurn => update_ending_turn(game),
    }
    Some(())
}
