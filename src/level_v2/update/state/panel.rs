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

pub fn make_action_preview_panel(actions: &Vec<&Action>, y: &mut f32) -> Panel {
    let mut builder = Panel::builder()
        .title("ACTIONS", WHITE)
        .size(UI_WIDTH, 0.0)
        .position(UI_ORIGIN.0, *y);

    for action in actions {
        builder = builder.text(action.name.to_string(), WHITE);
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
        .build();
    
    *y += panel.get_height() + PADDING;
    panel
}
