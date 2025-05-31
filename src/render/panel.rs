use crate::prelude::*;

pub fn make_unit_description_panel(unit: &Unit) -> Panel {
    Panel::builder(&unit.name.to_uppercase(), unit.glyph.color)
        .big_glyph(unit.glyph, 4.0)
        .meter("HP ", WHITE, unit.hp, unit.hp_max, RED)
        .build()
}

pub fn make_movement_panel(unit: &Unit, moves_left: u16) -> Panel {
    Panel::builder(&unit.name.to_uppercase(), unit.glyph.color)
        .big_glyph(unit.glyph, 4.0)
        .line(&format!("Movement: {}", moves_left), WHITE)
        .build()
}

pub fn make_action_menu_panel(unit: &Unit, actions: &Vec<Action>, selected_index: usize) -> Panel {
    let mut panel = Panel::builder(&unit.name.to_uppercase(), unit.glyph.color)
        .big_glyph(unit.glyph, 4.0)
        .line("Actions:", WHITE);

    for (i, action) in actions.iter().enumerate() {
        let color = if selected_index == i { WHITE } else { GRAY };
        let str = format!(" {}", action.name.to_uppercase());
        panel = panel.line(str, color);
    }

    panel.build()
}

pub fn make_action_description_panel(action: &Action) -> Panel {
    let mut panel = Panel::builder(action.name.to_uppercase(), WHITE);

    let range_str = match action.range {
        Range::SingleUnit {
            min_range,
            max_range,
        } => {
            if min_range == max_range {
                format!("Range {}", min_range)
            } else {
                format!("Range {}-{}", min_range, max_range)
            }
        }
    };

    panel = panel.line(&range_str, WHITE);
    panel = panel.line("Effects:", WHITE);

    for effect in action.effect_templates.iter() {
        match effect {
            EffectTemplate::Damage { min, max } => {
                let str = format!(" Damage: {}-{}", min, max);
                panel = panel.line(&str, RED);
            }
        };
    }

    panel.build()
}

pub fn make_no_targets_panel() -> Panel {
    Panel::builder("INFO", WHITE)
        .line("No targets!", WHITE)
        .build()
}
