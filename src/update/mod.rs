use std::collections::VecDeque;

use crate::model::*;

const TURN_START_DURATION: f32 = 1.0;
const NPC_MOVE_DURATION: f32 = 0.5;

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
        GameState::NpcSelectingMove => {
            let unit = game.active_unit()?;
            if let Some(behavior) = unit.npc_behavior.as_ref() {
                let path = (behavior.select_move)(unit, game);
                game.state = GameState::NpcExecutingMove { path, time: 0.0 };
            } else {
                game.state = GameState::EndingTurn;
            }
        }
        GameState::SelectingMove { moves_left } => {
            let dir = input::pressed_direction()?;
            let coord = game.active_unit()?.coord;
            let next_coord = coord.shift(dir);
            game.map.walkable(next_coord).then_some(())?;
            game.unit_at(next_coord).is_none().then_some(())?;

            game.state = GameState::ExecutingMove {
                next_coord,
                moves_left: moves_left - 1,
            };
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
                game.state = GameState::EndingTurn;
            }
        }
        GameState::NpcExecutingMove { path, time } => {
            if path.is_empty() {
                game.state = GameState::EndingTurn;
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
        GameState::EndingTurn => {
            game.next_turn();

            game.state = GameState::StartingTurn {
                time: TURN_START_DURATION,
            };
        }
    }
    Some(())
}
