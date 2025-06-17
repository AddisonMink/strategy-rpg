use macroquad::prelude::trace;

use super::INFO_PANEL_ORIGIN;
use super::INFO_PANEL_WIDTH;
use crate::engine::*;
use crate::level_model::*;

const PADDING: f32 = 10.0;

pub fn draw_state(level: &Level) {
    match &level.state {
        LevelState::Starting => {
            grid::draw_big_message("ESCAPE!".to_string(), WHITE);
        }
        LevelState::SelectingMove {
            valid_moves,
            path,
            action_previews,
        } => {
            for c in valid_moves.iter() {
                grid::draw_square(*c, WHITE.with_alpha(0.5));
            }
            for c in path.iter().flatten() {
                grid::draw_glyph(*c, Glyph::new('o', WHITE));
            }

            let action_preview_panel = action_preview_panel(action_previews);
            action_preview_panel.draw(INFO_PANEL_ORIGIN.0, INFO_PANEL_ORIGIN.1);
            let y = INFO_PANEL_ORIGIN.1 + action_preview_panel.get_height() + PADDING;
            draw_description_panels(level, y);
        }
        LevelState::SelectingAction {
            panel,
            selected_action,
            target_coords,
            ..
        } => {
            let mut y = INFO_PANEL_ORIGIN.1;

            panel.draw(INFO_PANEL_ORIGIN.0, y);
            y += panel.get_height() + PADDING;

            if let Some(action) = selected_action {
                let panel = action_description_panel(action);
                panel.draw(INFO_PANEL_ORIGIN.0, y);
                y += panel.get_height() + PADDING;
            }

            draw_description_panels(level, y);

            for coord in target_coords.iter().flatten() {
                grid::draw_square(*coord, WHITE.with_alpha(0.5));
            }
        }
        LevelState::SelectingSingleUnitTarget {
            action,
            targets,
            selected_target,
        } => {
            let mut y = INFO_PANEL_ORIGIN.1;

            let panel = action_description_panel(action);
            panel.draw(INFO_PANEL_ORIGIN.0, y);
            y += panel.get_height() + PADDING;

            for (coord, _) in targets.iter() {
                grid::draw_square(*coord, WHITE.with_alpha(0.5));
            }

            if let Some(target) = selected_target {
                grid::draw_square(*target, WHITE.with_alpha(0.5));
            }

            draw_description_panels(level, y);
        }
        LevelState::Success => {
            grid::draw_big_message("CLEAR!".to_string(), WHITE);
        }
        _ => draw_description_panels(level, INFO_PANEL_ORIGIN.1),
    }
}

fn draw_description_panels(level: &Level, mut y: f32) {
    let Some(coord) = grid::mouse_coord() else {
        return;
    };

    if let Some(unit) = level.unit_at(coord) {
        if level.player_vision.entity_visible(unit.entity) {
            let panel = unit_description_panel(unit);
            panel.draw(INFO_PANEL_ORIGIN.0, y);
            y += panel.get_height() + PADDING;
        }
    }

    let tile = level.map.tile(coord);
    if level.player_vision.tile_visible(coord) {
        let panel = tile_description_panel(tile);
        panel.draw(INFO_PANEL_ORIGIN.0, y);
        y += panel.get_height() + PADDING;
    }
}

fn action_preview_panel(actions: &Vec<(ItemAction, bool)>) -> Panel {
    let mut panel = Panel::builder("ACTIONS", WHITE).min_width(INFO_PANEL_WIDTH);

    for (action, valid) in actions {
        let alpha = if *valid { 1.0 } else { 0.25 };
        let color = action.item_color.with_alpha(alpha);
        panel = panel.line(action.action.name.to_string(), color);
    }

    panel.build()
}

fn tile_description_panel(tile: &Tile) -> Panel {
    let mut panel = Panel::builder(tile.name.to_string().to_uppercase(), tile.glyph.color)
        .min_width(INFO_PANEL_WIDTH);

    if !tile.walkable {
        panel = panel.line("BLOCKING", WHITE);
    }

    if !tile.transparent {
        panel = panel.line("OPAQUE", WHITE);
    }

    if tile.goal {
        panel = panel.line("GOAL", WHITE);
    }

    panel.build()
}

fn unit_description_panel(unit: &Unit) -> Panel {
    Panel::builder(unit.name.to_string().to_uppercase(), unit.glyph.color)
        .min_width(INFO_PANEL_WIDTH)
        .meter("HP ", WHITE, unit.hp, unit.hp_max, RED)
        .line(format!("Move {}", unit.movement), WHITE)
        .line(format!("Vision {}", unit.vision), WHITE)
        .build()
}

fn action_description_panel(action: &ItemAction) -> Panel {
    let title = action.action.name.to_string().to_uppercase();
    let title_color = action.item_color;
    let meter_label = format!("{} ", action.item_name.to_string());
    let meter_value = action.uses;
    let meter_max = action.uses_max;

    let range_text = match action.action.range {
        Range::SelfRange => "Self".to_string(),
        Range::SingleUnit { min, max } => format!("Unit {}-{}", min, max),
    };

    let mut panel = Panel::builder(title, title_color)
        .min_width(INFO_PANEL_WIDTH)
        .meter_diff(
            meter_label,
            title_color,
            meter_value,
            action.action.cost,
            meter_max,
            title_color,
        )
        .line(range_text, WHITE)
        .line("Effects", WHITE);

    for effect in action.action.effects.as_slice() {
        if let Some((desc, color)) = effect_template_desc(effect) {
            let text = format!(" {}", desc);
            panel = panel.line(text, color);
        }
    }

    panel.build()
}

fn effect_template_desc(effect: &EffectTemplate) -> Option<(String, Color)> {
    match effect {
        EffectTemplate::Damage { min, max } => Some((format!("Damage {}-{}", min, max), RED)),
        EffectTemplate::AddLightToEntity { radius, .. } => {
            Some((format!("Light {}", radius), ORANGE))
        }
        _ => None,
    }
}
