use macroquad::prelude::*;

pub fn mouse_pos() -> (f32, f32) {
    mouse_position()
}

pub fn mouse_clicked() -> bool {
    is_mouse_button_pressed(MouseButton::Left)
}

pub fn cancel_pressed() -> bool {
    is_key_pressed(KeyCode::Escape)
}

pub fn number_pressed() -> Option<usize> {
    if is_key_pressed(KeyCode::Key1) {
        Some(1)
    } else if is_key_pressed(KeyCode::Key2) {
        Some(2)
    } else if is_key_pressed(KeyCode::Key3) {
        Some(3)
    } else if is_key_pressed(KeyCode::Key4) {
        Some(4)
    } else if is_key_pressed(KeyCode::Key5) {
        Some(5)
    } else if is_key_pressed(KeyCode::Key6) {
        Some(6)
    } else if is_key_pressed(KeyCode::Key7) {
        Some(7)
    } else if is_key_pressed(KeyCode::Key8) {
        Some(8)
    } else if is_key_pressed(KeyCode::Key9) {
        Some(9)
    } else {
        None
    }
}
