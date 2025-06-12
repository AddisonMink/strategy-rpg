mod draw_animation;
mod draw_map;
mod draw_state;

use crate::level_model::*;
use draw_animation::draw_animation;
use draw_map::draw_map;
use draw_state::draw_state;

pub const INFO_PANEL_WIDTH: f32 = 200.0;
pub const INFO_PANEL_ORIGIN: (f32, f32) = (360.0, 10.0);

pub fn render_level(level: &Level) {
    draw_map(level);
    draw_state(level);
    draw_animation(level);
}
