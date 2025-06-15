use super::action;
use super::*;
use std::collections::VecDeque;

const MOVE_DELAY: f32 = 0.25;

pub fn transition(level: &mut Level) {
    let entity = level.turn_queue.front().unwrap();
    let unit = level.units.get(entity).unwrap();

    match unit.side {
        Side::Player => {
            let pos = level.positions.get(entity).unwrap();
            let accept = |c: Coord| level.map.tile(c).walkable && level.unit_at(c).is_none();
            let mut valid_moves = algorithm::flood_fill(pos.coord, unit.movement, accept);
            valid_moves.remove(&pos.coord);

            if valid_moves.is_empty() {
                level.state = LevelState::ResolvingMove;
            } else {
                level.state = LevelState::SelectingMove {
                    valid_moves,
                    path: None,
                    action_previews: vec![],
                };
            }
        }
        Side::NPC => {
            let behavior = level.behaviors.get(entity).unwrap();
            let path = (behavior.select_move)(&level).unwrap_or_default();
            enqueue_moves(level, path);
            level.state = LevelState::ResolvingMove;
        }
    }
}

pub fn update(level: &mut Level) {
    let LevelState::SelectingMove {
        valid_moves, path, ..
    } = &level.state
    else {
        return;
    };

    if (input::mouse_clicked() && path.is_none()) || input::cancel_pressed() {
        level.state = LevelState::ResolvingMove;
        return;
    }

    // If the mouse is clicked on a valid move, execute the move.
    if input::mouse_clicked() && path.is_some() {
        enqueue_moves(level, path.clone().unwrap());
        level.state = LevelState::ResolvingMove;
        return;
    }

    // If the mouse is not hovering over a valid move, clear the path.
    let Some(mouse_coord) = grid::mouse_coord().filter(|c| valid_moves.contains(c)) else {
        level.state = LevelState::SelectingMove {
            valid_moves: valid_moves.clone(),
            path: None,
            action_previews: vec![],
        };
        return;
    };

    // If the mouse has not moved since the last update, do nothing.
    let last_coord_opt = path.as_ref().and_then(|p| p.iter().last());
    if last_coord_opt.is_some_and(|last| last == &mouse_coord) {
        return;
    }

    let (pos, _) = level.active_unit_with_position().unwrap();
    let accept = |c: Coord| level.map.tile(c).walkable && level.unit_at(c).is_none();
    let goal = |c: Coord| c == mouse_coord;
    let new_path = algorithm::breadth_first_search(pos.coord, accept, goal);
    let action_previews = action::player_actions(level, mouse_coord);

    level.state = LevelState::SelectingMove {
        valid_moves: valid_moves.clone(),
        path: Some(new_path),
        action_previews,
    };
}

fn enqueue_moves(level: &mut Level, path: VecDeque<Coord>) {
    let entity = level.turn_queue.front().unwrap().clone();

    for coord in path.iter().copied() {
        level.effect_queue.push_back(Effect::Move { entity, coord });
        level.effect_queue.push_back(Effect::Sleep {
            duration: MOVE_DELAY,
        });
    }

    if !path.is_empty() {
        level.effect_queue.pop_back(); // Remove the last sleep effect to avoid an extra delay after the last move.
    }
}
