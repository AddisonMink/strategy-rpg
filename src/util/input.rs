use super::direction::Direction;
use macroquad::prelude::*;

pub fn pressed_cancel() -> bool {
    is_key_pressed(KeyCode::K)
}

pub fn pressed_direction() -> Option<Direction> {
    if is_key_pressed(KeyCode::W) {
        Some(Direction::Up)
    } else if is_key_pressed(KeyCode::S) {
        Some(Direction::Down)
    } else if is_key_pressed(KeyCode::A) {
        Some(Direction::Left)
    } else if is_key_pressed(KeyCode::D) {
        Some(Direction::Right)
    } else {
        None
    }
}
