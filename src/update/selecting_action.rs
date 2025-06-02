use crate::prelude::*;

use super::selecting_single_unit_target::to_selecting_single_unit_target;

pub fn to_selecting_action(game: &mut Game) {
    game.state = GameState::SelectingAction {
        actions: vec![basic_attack()],
        selected_index: 0,
    };
}

pub fn update_selecting_action(game: &mut Game) {
    let GameState::SelectingAction {
        actions,
        selected_index,
    } = &mut game.state
    else {
        return;
    };

    if input::pressed_confirm() {
        let action = &actions[*selected_index].clone();
        to_selecting_single_unit_target(game, action.clone());
    } else if input::pressed_cancel() {
        game.state = GameState::EndingTurn;
    }
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
