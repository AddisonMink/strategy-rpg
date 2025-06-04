use super::direction::Direction;
use macroquad::prelude::*;

pub fn mouse_clicked() -> bool {
    is_mouse_button_pressed(MouseButton::Left)
}

pub fn pressed_confirm() -> bool {
    is_key_pressed(KeyCode::J)
}

pub fn pressed_cancel() -> bool {
    is_key_pressed(KeyCode::K)
}

pub fn mouse_pos() -> (f32, f32) {
    mouse_position()
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
