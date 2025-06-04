use macroquad::prelude::trace;

use super::model::*;
use crate::engine::*;

const DESCRIPTION_X: f32 = 360.0;

pub fn draw_battle(battle: &Battle) {
    let mouse_coord_opt = grid::mouse_coord();
    draw_map(battle);
    draw_state(battle);

    if let Some(coord) = mouse_coord_opt {
        grid::draw_square(coord, WHITE.with_alpha(0.25));
    }

    draw_description_panels(battle, mouse_coord_opt);
}

fn draw_map(battle: &Battle) {
    grid::draw_frame("MAP");

    for y in 0..Map::HEIGHT {
        for x in 0..Map::WIDTH {
            let coord = Coord::new(x, y);
            let tile = battle.map.tile(coord);
            let unit_opt = battle.unit_at(coord);

            let glyph = if let Some(unit) = unit_opt {
                unit.glyph
            } else {
                tile.glyph
            };

            if let Some(bg_color) = tile.bg_color {
                grid::draw_square(coord, bg_color);
            }

            grid::draw_glyph(coord, glyph);
        }
    }
}

fn draw_description_panels(battle: &Battle, mouse_coord_opt: Option<Coord>) {
    let Some(coord) = mouse_coord_opt else {
        return;
    };

    let tile = battle.map.tile(coord);
    let unit_opt = battle.unit_at(coord);
    let tile_panel = make_tile_description_panel(tile);
    let unit_panel_opt = unit_opt.map(make_unit_description_panel);
    let mut y = 10.0;

    if let Some(unit_panel) = unit_panel_opt {
        unit_panel.draw(DESCRIPTION_X, y);
        y += unit_panel.get_height() + 10.0;
    }
    tile_panel.draw(DESCRIPTION_X, y);
}

fn make_unit_description_panel(unit: &Unit) -> Panel {
    Panel::builder(unit.name.to_string().to_uppercase(), unit.glyph.color)
        .min_width(200.0)
        .build()
}

fn make_tile_description_panel(tile: &Tile) -> Panel {
    Panel::builder(tile.name.to_string().to_uppercase(), tile.glyph.color)
        .min_width(200.0)
        .build()
}

fn draw_state(battle: &Battle) {
    match &battle.state {
        BattleState::SelectingMove { valid_moves, path } => {
            for coord in valid_moves {
                grid::draw_square(*coord, WHITE.with_alpha(0.5));
            }
            if let Some(path) = path {
                for coord in path {
                    grid::draw_glyph(*coord, Glyph::new('o', WHITE));
                }
            }
        }
        _ => {}
    };
}
