use std::collections::VecDeque;

use crate::model::*;

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
                game.state = GameState::EndingTurn;
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
        GameState::SelectingAction { .. } => {}
        GameState::NpcSelectingAction => {
            game.state = GameState::ExecutingEffects {
                effects: VecDeque::new(),
            };
        }
        GameState::ExecutingEffects { .. } => {
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
