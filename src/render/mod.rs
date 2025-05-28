pub mod draw_grid;
pub mod draw_map;
pub mod panel;
pub mod util;

pub use draw_map::*;
use macroquad::prelude::*;
pub use panel::*;
pub use util::*;

use crate::model::*;

pub fn draw_game(game: &Game, flicker: f32) -> Option<()> {
    draw_grid::draw_frame_panel();
    draw_map(game, flicker);

    match game.state {
        GameState::StartingTurn { .. } => {
            let unit = game.active_unit()?;

            let panel = &Panel::builder("", WHITE)
                .line(&unit.name, unit.glyph.color)
                .build();

            draw_map(game, flicker);
            draw_grid::draw_panel_centered(panel);
        }
        _ => {}
    }

    Some(())
}
