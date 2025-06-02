mod animation;
mod draw_grid;
mod draw_map;
mod panel;

use crate::prelude::*;
use animation::*;
use draw_map::*;
use panel::*;

const INFO_PANEL_X: f32 = 552.0;
const INFO_PANEL_Y: f32 = 10.0;

pub fn draw_game(game: &Game, flicker: f32) -> Option<()> {
    draw_grid::draw_frame_panel();
    draw_map(game, flicker);

    match &game.state {
        GameState::StartingTurn { .. } => {
            let unit = game.active_unit()?;
            let player_can_see = game.any_player_can_see(unit.coord);

            let (name, color) = if player_can_see {
                (unit.name.to_uppercase(), unit.glyph.color)
            } else {
                ("???".to_string(), WHITE)
            };

            let str = format!("{}'s Turn", name);

            let panel = &Panel::builder("INFO", WHITE).line(&str, color).build();

            draw_map(game, flicker);
            draw_grid::draw_panel_centered(panel);
        }
        GameState::SelectingMove { moves_left } => {
            let unit = game.active_unit()?;
            let panel = make_movement_panel(unit, *moves_left);
            draw_map(game, flicker);
            panel.draw(INFO_PANEL_X, INFO_PANEL_Y);
        }
        GameState::SelectingAction {
            actions,
            selected_index,
        } => {
            let unit = game.active_unit()?;
            let action = &actions[*selected_index];
            let action_menu_panel = make_action_menu_panel(unit, &actions, *selected_index);
            let action_description_y = INFO_PANEL_Y + action_menu_panel.get_height() + 10.0;
            let action_description_panel = make_action_description_panel(action);
            draw_map(game, flicker);
            action_menu_panel.draw(INFO_PANEL_X, INFO_PANEL_Y);
            action_description_panel.draw(INFO_PANEL_X, action_description_y);
        }
        GameState::SelectingSingleUnitTarget {
            action,
            targets,
            selected_index,
        } => {
            let action_description_panel = make_action_description_panel(action);
            let next_panel_y = INFO_PANEL_Y + action_description_panel.get_height() + 10.0;
            action_description_panel.draw(INFO_PANEL_X, INFO_PANEL_Y);

            if targets.is_empty() {
                let no_targets_panel = make_no_targets_panel();
                no_targets_panel.draw(INFO_PANEL_X, next_panel_y);
            } else {
                let selected_id = targets[*selected_index];
                let selected_unit = game.unit(selected_id).unwrap();
                let unit_description_panel = make_unit_description_panel(selected_unit);

                for target_id in targets.iter() {
                    let coord = game.unit(*target_id).unwrap().coord;
                    draw_grid::draw_square(coord, WHITE.with_alpha(0.25));
                }

                draw_grid::draw_square(selected_unit.coord, WHITE.with_alpha(0.5));
                unit_description_panel.draw(INFO_PANEL_X, next_panel_y);
            }
        }
        GameState::ShowingAnimations { animations, .. } => {
            if let Some(animation) = animations.front() {
                draw_animation(game, animation);
            }
        }
        _ => {}
    }

    Some(())
}
