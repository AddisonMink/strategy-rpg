use std::collections::VecDeque;

use crate::prelude::*;

use super::{executing_effects::to_executing_effects, selecting_action::to_selecting_action};

pub fn to_selecting_single_unit_target(game: &mut Game, action: Action) {
    let Range::SingleUnit {
        min_range,
        max_range,
    } = action.range;
    let targets = find_targets_in_range(game, min_range, max_range);

    game.state = GameState::SelectingSingleUnitTarget {
        action,
        targets,
        selected_index: 0,
    };
}

pub fn update_selecting_single_unit_target(game: &mut Game) {
    let GameState::SelectingSingleUnitTarget {
        action,
        targets,
        selected_index,
    } = &mut game.state
    else {
        return;
    };

    if input::pressed_confirm() {
        if let Some(&target) = targets.get(*selected_index) {
            let effects = compile_action(action.clone(), target);
            to_executing_effects(game, effects);
        }
    } else if input::pressed_cancel() {
        to_selecting_action(game);
    }
}

fn find_targets_in_range(game: &Game, min_range: u16, max_range: u16) -> Vec<UnitId> {
    let unit = game.active_unit().unwrap();
    let coord = unit.coord;

    let is_valid_target = |c: Coord| {
        let distance = coord.manhattan_distance(c);
        game.player_can_see(unit.id, coord) && distance >= min_range && distance <= max_range
    };

    game.unit_iter()
        .filter(|u| !u.is_player && is_valid_target(u.coord))
        .map(|u| u.id)
        .collect()
}

fn compile_action(action: Action, target: UnitId) -> VecDeque<Effect> {
    action
        .effect_templates
        .into_iter()
        .map(|template| match template {
            EffectTemplate::Damage { min, max } => Effect::Damage { min, max, target },
        })
        .collect()
}
