use macroquad::prelude::trace;

use crate::model::*;

const TURN_START_DURATION: f32 = 1.0;

pub fn update_game(game: &mut Game, delta_time: f32) -> Option<()> {
    match game.state {
        GameState::Start => {
            game.state = GameState::StartingTurn {
                time: TURN_START_DURATION,
            }
        }
        GameState::StartingTurn { time } => {
            if time > 0.0 {
                game.state = GameState::StartingTurn {
                    time: time - delta_time,
                };
            } else {
                let unit = game.active_unit()?;
                game.state = GameState::SelectingMove {
                    moves_left: game.active_unit()?.movement,
                };
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
        GameState::EndingTurn => {
            trace!("Ending turn");
            game.next_turn();

            game.state = GameState::StartingTurn {
                time: TURN_START_DURATION,
            };
        }
    }
    Some(())
}
