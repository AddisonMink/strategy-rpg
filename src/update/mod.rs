use macroquad::rand::gen_range;

use crate::prelude::*;
use std::{collections::VecDeque, vec};

const TURN_START_DURATION: f32 = 0.5;
const NPC_MOVE_DURATION: f32 = 0.2;

pub fn update_game(game: &mut Game, delta_time: f32) -> Option<()> {
    match game.state.clone() {
        GameState::Start => {
            game.state = GameState::StartingTurn {
                time: TURN_START_DURATION,
            }
        }
        GameState::StartingTurn { time } => {
            if time <= 0.0 {
                let unit = game.active_unit()?;
                if unit.is_player {
                    game.state = GameState::SelectingMove {
                        moves_left: unit.movement,
                    };
                } else {
                    game.state = GameState::NpcSelectingMove;
                }
            } else {
                game.state = GameState::StartingTurn {
                    time: time - delta_time,
                };
            }
        }
        GameState::SelectingMove { moves_left } => {
            if input::pressed_cancel() {
                game.state = GameState::SelectingAction {
                    actions: vec![basic_attack()],
                    selected_index: 0,
                }
            } else if let Some(dir) = input::pressed_direction() {
                let coord = game.active_unit()?.coord;
                let next_coord = coord.shift(dir);
                game.map.walkable(next_coord).then_some(())?;
                game.unit_at(next_coord).is_none().then_some(())?;

                game.state = GameState::ExecutingMove {
                    next_coord,
                    moves_left: moves_left - 1,
                };
            }
        }
        GameState::NpcSelectingMove => {
            let unit = game.active_unit()?;

            let path = unit
                .npc_behavior
                .as_ref()
                .and_then(|b| (b.select_move)(unit, game))
                .unwrap_or(VecDeque::new());

            game.state = GameState::NpcExecutingMove { path, time: 0.0 };
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
                game.state = GameState::SelectingMove { moves_left };
            } else {
                game.state = GameState::SelectingAction {
                    actions: vec![basic_attack()],
                    selected_index: 0,
                }
            }
        }
        GameState::NpcExecutingMove { path, time } => {
            if path.is_empty() {
                game.state = GameState::NpcSelectingAction;
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

                let Range::SingleUnit {
                    min_range,
                    max_range,
                } = action.range;

                let targets = find_targets_in_range(game, min_range, max_range);

                game.state = GameState::SelectingSingleUnitTarget {
                    action: action.clone(),
                    targets,
                    selected_index: 0,
                };
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
                    game.state = GameState::ExecutingEffects { effects };
                }
            } else if input::pressed_cancel() {
                game.state = GameState::SelectingAction {
                    actions: vec![basic_attack()],
                    selected_index: 0,
                };
            }
        }
        GameState::NpcSelectingAction => {
            game.state = GameState::ExecutingEffects {
                effects: VecDeque::new(),
            };
        }
        GameState::ExecutingEffects { effects } => {
            for effect in effects {
                execute_effect(game, effect);
            }
            game.state = GameState::EndingTurn;
        }
        GameState::EndingTurn => {
            game.next_turn();

            game.state = GameState::StartingTurn {
                time: TURN_START_DURATION,
            };
        }
    }
    Some(())
}

fn basic_attack() -> Action {
    Action {
        name: "Attack".to_string(),
        range: Range::SingleUnit {
            min_range: 1,
            max_range: 1,
        },
        effect_templates: vec![EffectTemplate::Damage { min: 1, max: 4 }],
    }
}

fn find_targets_in_range(game: &Game, min_range: u16, max_range: u16) -> Vec<UnitId> {
    let unit = game.active_unit().unwrap();
    let coord = unit.coord;

    let is_valid_target = |c: Coord| {
        let distance = coord.manhattan_distance(c);
        game.player_can_see(unit.id, coord) && distance >= min_range && distance <= max_range
    };

    game.unit_iter()
        .filter(|u| !u.is_player && is_valid_target(u.coord))
        .map(|u| u.id)
        .collect()
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
