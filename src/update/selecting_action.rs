use crate::prelude::*;

pub fn to_selecting_action(game: &mut Game) {
    game.state = GameState::SelectingAction {
        actions: vec![basic_attack()],
        selected_index: 0,
    };
}

fn basic_attack() -> Action {
    Action {
        name: "Attack".to_string(),
        range: Range::SingleUnit {
            min_range: 1,
            max_range: 1,
        },
        effect_templates: vec![EffectTemplate::Damage { min: 1, max: 4 }],
    }
}
