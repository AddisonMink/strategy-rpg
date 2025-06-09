mod animation;
mod panel;

use super::model::*;
use crate::engine::{color::mix_color, *};
use panel::*;

const DESCRIPTION_X: f32 = 360.0;

pub fn draw_battle(battle: &Battle) {
    let mouse_coord_opt = grid::mouse_coord().filter(|c| battle.get_light_grid().visible(*c));
    draw_map(battle);
    draw_state(battle, mouse_coord_opt);

    if let Some(coord) = mouse_coord_opt {
        grid::draw_square(coord, WHITE.with_alpha(0.25));
    }
}

fn draw_map(battle: &Battle) {
    grid::draw_frame("MAP");

    let animating_unit_id = find_animating_unit_id(battle);
    let light_grid = battle.get_light_grid();

    for y in 0..Map::HEIGHT {
        for x in 0..Map::WIDTH {
            let coord = Coord::new(x, y);
            if light_grid.visible(coord) {
                let tile = battle.map.tile(coord);
                let light_color = light_grid.color_at(coord);
                let unoccupied = battle.unit_at(coord).is_none();

                if let Some(bg_color) = tile.bg_color {
                    let color = mix_color(bg_color, light_color, 0.5);
                    grid::draw_square(coord, color);
                }
                if unoccupied {
                    let color = mix_color(tile.glyph.color, light_color, 0.5);
                    let glyph = Glyph::new(tile.glyph.symbol, color);
                    grid::draw_glyph(coord, glyph);
                }
            }
        }
    }

    for unit in battle.unit_iter() {
        let visible = !animating_unit_id.is_some_and(|id| id == unit.id)
            && (unit.side == Side::Player || light_grid.unit_visible(unit.id));

        if visible {
            let light_color = light_grid.color_at(unit.coord);
            let color = mix_color(unit.glyph.color, light_color, 0.5);
            let glyph = Glyph::new(unit.glyph.symbol, color);
            grid::draw_glyph(unit.coord, glyph);
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
    let unit_opt = battle
        .unit_at(coord)
        .filter(|u| u.side == Side::Player || battle.light_grid.unit_visible(u.id));
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
            } else {
                draw_description_panels(battle, mouse_coord_opt, None);
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
        BattleState::ExecutingEffects { animations, .. } => {
            if let Some(animation) = animations.front() {
                animation::draw_animation(battle, animation);
            }
            draw_description_panels(battle, mouse_coord_opt, None);
        }
        _ => draw_description_panels(battle, mouse_coord_opt, None),
    };
}

fn find_animating_unit_id(battle: &Battle) -> Option<UnitId> {
    let BattleState::ExecutingEffects { animations, .. } = &battle.state else {
        return None;
    };

    let animation = animations.front()?;

    let AnimationKind::Attack { unit_id, .. } = animation.kind else {
        return None;
    };

    Some(unit_id)
}
