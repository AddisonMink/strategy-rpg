use super::*;

pub fn make_unit_description_panel(unit: &Unit) -> Panel {
    Panel::builder(unit.name.to_string().to_uppercase(), unit.glyph.color)
        .min_width(200.0)
        .build()
}

pub fn make_tile_description_panel(tile: &Tile) -> Panel {
    Panel::builder(tile.name.to_string().to_uppercase(), tile.glyph.color)
        .min_width(200.0)
        .build()
}

pub fn make_action_preview_panel(battle: &Battle, origin: Coord) -> Panel {
    let unit = battle.active_unit().expect("No active unit");
    let actions = unit.actions();

    let mut panel = Panel::builder("ACTION", WHITE).min_width(200.0);
    for action in actions {
        let alpha = if action.has_valid_targets(battle, unit.id, origin) {
            1.0
        } else {
            0.5
        };
        panel = panel.line(action.name.to_string(), WHITE.with_alpha(alpha));
    }

    panel.build()
}

pub fn make_action_description_panel(action: &Action) -> Panel {
    let mut panel = Panel::builder(action.name.to_string().to_uppercase(), WHITE).min_width(200.0);

    match &action.range {
        Range::SingleUnit { min, max } => {
            panel = panel.line(format!("Range {}-{}", min, max), WHITE);
        }
    }

    panel = panel.line("Effects:", WHITE);

    for effect in action.effects.as_slice() {
        match effect {
            EffectTemplate::Damage { min, max } => {
                panel = panel.line(format!(" Damage {}-{}", min, max), WHITE);
            }
        }
    }

    panel.build()
}
