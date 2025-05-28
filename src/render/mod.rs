pub mod draw_grid;
pub mod draw_map;
pub mod panel;
pub mod util;

pub use draw_map::*;
use macroquad::prelude::*;
pub use panel::*;
pub use util::*;

use crate::model::*;

const INFO_PANEL_X: f32 = 552.0;
const INFO_PANEL_Y: f32 = 10.0;

pub fn draw_game(game: &Game, flicker: f32) -> Option<()> {
    draw_grid::draw_frame_panel();
    draw_map(game, flicker);

    match game.state {
        GameState::StartingTurn { .. } => {
            let unit = game.active_unit()?;
            let str = format!("{}'s Turn", unit.name);

            let panel = &Panel::builder("INFO", WHITE)
                .line(&str, unit.glyph.color)
                .build();

            draw_map(game, flicker);
            draw_grid::draw_panel_centered(panel);
        }
        GameState::SelectingMove { moves_left } => {
            let unit = game.active_unit()?;

            let panel = &Panel::builder(&unit.name.to_uppercase(), unit.glyph.color)
                .big_glyph(unit.glyph, 4.0)
                .line(&format!("Movement: {}", moves_left), WHITE)
                .build();

            draw_map(game, flicker);
            panel.draw(INFO_PANEL_X, INFO_PANEL_Y);
        }
        _ => {}
    }

    Some(())
}
