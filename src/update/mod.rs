use ending_turn::to_ending_turn;
use executing_effects::to_executing_effects;
use executing_move::to_executing_move;
use macroquad::rand::gen_range;
use npc_executing_move::to_npc_executing_move;
use npc_selecting_action::to_npc_selecting_action;
use npc_selecting_move::to_npc_selecting_move;
use selecting_action::to_selecting_action;
use selecting_move::{to_selecting_move, to_selecting_move_ex};
use selecting_single_unit_target::to_selecting_single_unit_target;
use starting_turn::to_starting_turn;

mod ending_turn;
mod executing_effects;
mod executing_move;
mod npc_executing_move;
mod npc_selecting_action;
mod npc_selecting_move;
mod selecting_action;
mod selecting_move;
mod selecting_single_unit_target;
mod starting_turn;

use crate::prelude::*;
use std::{clone, collections::VecDeque};

const NPC_MOVE_DURATION: f32 = 0.2;

pub fn update_game(game: &mut Game, delta_time: f32) -> Option<()> {
    match game.state.clone() {
        GameState::Start => to_starting_turn(game),
        GameState::StartingTurn { time } => {
            if time <= 0.0 {
                let unit = game.active_unit()?;
                if unit.is_player {
                    to_selecting_move(game);
                } else {
                    to_npc_selecting_move(game);
                }
            } else {
                game.state = GameState::StartingTurn {
                    time: time - delta_time,
                };
            }
        }
        GameState::SelectingMove { moves_left } => {
            if input::pressed_cancel() {
                to_selecting_action(game);
            } else if let Some(dir) = input::pressed_direction() {
                let coord = game.active_unit()?.coord;
                let next_coord = coord.shift(dir);

                game.map.walkable(next_coord).then_some(())?;
                game.unit_at(next_coord).is_none().then_some(())?;
                to_executing_move(game, next_coord, moves_left - 1);
            }
        }
        GameState::NpcSelectingMove => {
            let unit = game.active_unit()?;

            let path = unit
                .npc_behavior
                .as_ref()
                .and_then(|b| (b.select_move)(unit, game))
                .unwrap_or(VecDeque::new());

            to_npc_executing_move(game, path);
        }
        GameState::ExecutingMove {
            next_coord,
            moves_left,
        } => {
            let unit = game.active_unit_mut()?;
            unit.coord = next_coord;

            if unit.light.is_some() {
                game.light_grid = LightGrid::new(game);
            }

            if moves_left > 0 {
                to_selecting_move_ex(game, moves_left);
            } else {
                to_selecting_action(game);
            }
        }
        GameState::NpcExecutingMove { path, time } => {
            if path.is_empty() {
                to_npc_selecting_action(game);
            } else if time <= 0.0 {
                let next_coord = path.front().copied().unwrap();
                let unit = game.active_unit_mut()?;
                let new_path: VecDeque<Coord> = path.iter().copied().skip(1).collect();

                unit.coord = next_coord;
                if unit.light.is_some() {
                    game.light_grid = LightGrid::new(game);
                }

                game.state = GameState::NpcExecutingMove {
                    path: new_path,
                    time: NPC_MOVE_DURATION,
                };
            } else {
                game.state = GameState::NpcExecutingMove {
                    path,
                    time: time - delta_time,
                };
            }
        }
        GameState::SelectingAction {
            actions,
            selected_index,
        } => {
            if input::pressed_confirm() {
                let action = &actions[selected_index];
                to_selecting_single_unit_target(game, action.clone());
            } else if input::pressed_cancel() {
                game.state = GameState::EndingTurn;
            }
        }
        GameState::SelectingSingleUnitTarget {
            action,
            targets,
            selected_index,
        } => {
            if input::pressed_confirm() {
                if let Some(&target) = targets.get(selected_index) {
                    let effects = compile_actions(action, target);
                    to_executing_effects(game, effects);
                }
            } else if input::pressed_cancel() {
                to_selecting_action(game);
            }
        }
        GameState::NpcSelectingAction => {
            to_executing_effects(game, VecDeque::new());
        }
        GameState::ExecutingEffects { effects } => {
            for effect in effects {
                execute_effect(game, effect);
            }
            to_ending_turn(game);
        }
        GameState::EndingTurn => {
            game.next_turn();
            to_starting_turn(game);
        }
    }
    Some(())
}

fn compile_actions(action: Action, target: UnitId) -> VecDeque<Effect> {
    action
        .effect_templates
        .into_iter()
        .map(|template| match template {
            EffectTemplate::Damage { min, max } => Effect::Damage { min, max, target },
        })
        .collect()
}

fn execute_effect(game: &mut Game, effect: Effect) -> Option<()> {
    match effect {
        Effect::Damage { min, max, target } => {
            let unit = game.unit_mut(target)?;
            let amount = roll(min, max);
            unit.hp = unit.hp.saturating_sub(amount);
        }
    }
    Some(())
}

fn roll(min: u16, max: u16) -> u16 {
    // rool twice and return the average
    let roll1 = gen_range(min, max + 1);
    let roll2 = gen_range(min, max + 1);
    (roll1 + roll2) / 2
}
