mod panel;

use super::model::*;
use crate::engine::*;
use panel::*;

const DESCRIPTION_X: f32 = 360.0;

pub fn draw_battle(battle: &Battle) {
    let mouse_coord_opt = grid::mouse_coord();
    draw_map(battle);
    draw_state(battle, mouse_coord_opt);

    if let Some(coord) = mouse_coord_opt {
        grid::draw_square(coord, WHITE.with_alpha(0.25));
    }
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

fn draw_description_panels(
    battle: &Battle,
    mouse_coord_opt: Option<Coord>,
    action_preview_origin: Option<Coord>,
) {
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
    y += tile_panel.get_height() + 10.0;

    if let Some(action_preview_origin) = action_preview_origin {
        let action_preview_panel = make_action_preview_panel(battle, action_preview_origin);
        action_preview_panel.draw(DESCRIPTION_X, y);
    }
}

fn draw_state(battle: &Battle, mouse_coord_opt: Option<Coord>) {
    match &battle.state {
        BattleState::SelectingMove { valid_moves, path } => {
            for coord in valid_moves {
                grid::draw_square(*coord, WHITE.with_alpha(0.5));
            }
            if let Some(path) = path {
                for coord in path {
                    grid::draw_glyph(*coord, Glyph::new('o', WHITE));
                }
                let action_preview_origin = path.back().copied();
                draw_description_panels(battle, mouse_coord_opt, action_preview_origin);
            }
        }
        BattleState::SelectingAction {
            actions: valid_actions,
            panel,
            selected_index,
        } => {
            let unit = battle.active_unit().expect("No active unit");
            let selected_action_opt = selected_index.and_then(|i| valid_actions.get(i));

            let coords_in_ragne = selected_action_opt
                .map(|action| action.range.coords_in_range(battle, unit.coord))
                .unwrap_or_default();

            let mut y = 10.0;
            panel.draw(DESCRIPTION_X, y);
            y += panel.get_height() + 10.0;

            if let Some(action) = selected_action_opt {
                let action_panel = make_action_description_panel(action);
                action_panel.draw(DESCRIPTION_X, y);
            }

            for coord in coords_in_ragne {
                grid::draw_square(coord, WHITE.with_alpha(0.5));
            }
        }
        BattleState::SelectingSingleUnitTarget {
            action,
            targets,
            selected_target,
        } => {
            let target_opt = selected_target.and_then(|id| battle.unit(id));
            let action_panel_y = 10.0;
            let action_panel = make_action_description_panel(action);
            action_panel.draw(DESCRIPTION_X, action_panel_y);

            if let Some(target) = target_opt {
                let unit_panel_y = action_panel_y + action_panel.get_height() + 10.0;
                let unit_panel = make_unit_description_panel(target);
                unit_panel.draw(DESCRIPTION_X, unit_panel_y);
            }

            for id in targets.iter() {
                if let Some(unit) = battle.unit(*id) {
                    let coord = unit.coord;
                    let alpha = if selected_target.is_some_and(|i| i == *id) {
                        0.5
                    } else {
                        0.25
                    };
                    grid::draw_square(coord, WHITE.with_alpha(alpha));
                }
            }
        }
        _ => draw_description_panels(battle, mouse_coord_opt, None),
    };
}
