use super::*;
use crate::constants::*;

pub fn make_cancel_button(y: &mut f32) -> Button {
    let button = Button::builder("Cancel")
        .min_size(UI_WIDTH, 0.0)
        .position(UI_ORIGIN.0, *y)
        .build();
    *y += button.size().1 + PADDING;
    button
}

pub fn make_action_preview_panel(world: &World, player_pos: Option<Coord>, y: &mut f32) -> Panel {
    let Some(unit) = world.active_unit() else {
        panic!("No active unit.");
    };

    let mut builder = Panel::builder()
        .title("ACTIONS", WHITE)
        .size(UI_WIDTH, 0.0)
        .position(UI_ORIGIN.0, *y);

    for action in unit.actions() {
        let valid = if let Some(origin) = player_pos {
            action
                .action
                .find_targets_from(world, unit, origin)
                .is_some()
        } else {
            false
        };

        let color = if valid { WHITE } else { GRAY };

        builder = builder.selectable_text(action.action.name.to_string(), color);
    }

    let panel = builder.build();
    *y += panel.get_height() + PADDING;
    panel
}

pub fn make_tile_description_panel(tile: &Tile, y: &mut f32) -> Panel {
    let mut builder = Panel::builder()
        .title(tile.name.as_str().to_uppercase(), tile.glyph.color)
        .size(UI_WIDTH, 0.0)
        .position(UI_ORIGIN.0, *y);

    if !tile.walkable {
        builder = builder.text("Blocking", WHITE);
    }

    if !tile.transparent {
        builder = builder.text("Opaque", WHITE);
    }

    let panel = builder.build();
    *y += panel.get_height() + PADDING;
    panel
}

pub fn make_unit_description_panel(unit: &Unit, y: &mut f32) -> Panel {
    let panel = Panel::builder()
        .title(
            unit.data().name.as_str().to_uppercase(),
            unit.data().glyph.color,
        )
        .size(UI_WIDTH, 0.0)
        .position(UI_ORIGIN.0, *y)
        .text(format!("Vision: {}", unit.data().vision), WHITE)
        .text(format!("Movement: {}", unit.data().movement), WHITE)
        .labeled_meter("HP:", unit.hp, 0, unit.data().hp_max, RED)
        .build();

    *y += panel.get_height() + PADDING;
    panel
}

pub fn make_action_description_panel(action: &ItemAction, y: &mut f32) -> Panel {
    let mut builder = Panel::builder()
        .title(
            action.action.name.to_string().to_uppercase(),
            action.item_color,
        )
        .text(action.item_name.as_str().to_uppercase(), action.item_color)
        .size(UI_WIDTH, 0.0)
        .position(UI_ORIGIN.0, *y);

    let range_str = match action.action.range {
        ActionRange::Enemy {
            min_range,
            max_range,
        } => {
            if min_range == max_range {
                format!("Enemy (Rng {})", min_range)
            } else {
                format!("Enemy (Rng {}-{})", min_range, max_range)
            }
        }
    };

    builder = builder.text(range_str, WHITE);
    builder = builder.text("Effects:", WHITE);

    for effect in action.action.effects.iter() {
        match effect {
            ActionEffect::Damage { min, max } => {
                if min == max {
                    builder = builder.text(format!(" Damage: {}", min), RED);
                } else {
                    builder = builder.text(format!(" Damage: {}-{}", min, max), RED);
                }
            }
            _ => {}
        };
    }

    let panel = builder.build();
    *y += panel.get_height() + PADDING;
    panel
}

pub fn make_action_list_panel(actions: &Vec<ItemAction>, y: &mut f32) -> Panel {
    let mut builder = Panel::builder()
        .title("ACTIONS", WHITE)
        .size(UI_WIDTH, 0.0)
        .position(UI_ORIGIN.0, *y);

    for (i, action) in actions.iter().enumerate() {
        let text = format!("{}. {}", i + 1, action.action.name);
        builder = builder.selectable_text(text, action.item_color);
    }

    let panel = builder.build();
    *y += panel.get_height() + PADDING;
    panel
}
