use super::INFO_PANEL_ORIGIN;
use super::INFO_PANEL_WIDTH;
use crate::battle::draw;
use crate::engine::*;
use crate::level_model::*;

const PADDING: f32 = 10.0;

pub fn draw_state(level: &Level) {
    match &level.state {
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

fn action_preview_panel(actions: &Vec<ActionPreview>) -> Panel {
    let mut panel = Panel::builder("ACTIONS", WHITE).min_width(INFO_PANEL_WIDTH);

    for action in actions {
        let color = if action.valid { WHITE } else { GRAY };
        panel = panel.line(action.name.to_string(), color);
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

    panel.build()
}

fn unit_description_panel(unit: &Unit) -> Panel {
    Panel::builder(unit.name.to_string().to_uppercase(), unit.glyph.color)
        .min_width(INFO_PANEL_WIDTH)
        .meter("HP", WHITE, unit.hp, unit.hp_max, RED)
        .line(format!("MOVE {}", unit.movement), WHITE)
        .line(format!("VISION {}", unit.vision), WHITE)
        .build()
}

fn action_description_panel(action: &Action) -> Panel {
    let mut panel =
        Panel::builder(action.name.to_string().to_uppercase(), WHITE).min_width(INFO_PANEL_WIDTH);

    let range_text = match action.range {
        Range::SelfRange => "SELF".to_string(),
        Range::SingleUnit { min, max } => format!("UNIT {}-{}", min, max),
    };
    panel = panel.line(range_text, WHITE);

    panel = panel.line("EFFECTS", WHITE);
    for effect in action.effects.as_slice() {
        let (text, color) = match effect {
            EffectTemplate::Damage { min, max } => (format!("DAMAGE {}-{}", min, max), RED),
        };
        panel = panel.line(format!(" {}", text), color);
    }

    panel.build()
}
