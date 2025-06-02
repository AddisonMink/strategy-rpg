use std::collections::VecDeque;

use crate::prelude::*;

use super::ending_turn::to_ending_turn;

pub fn to_executing_effects(game: &mut Game, effects: VecDeque<Effect>) {
    game.state = GameState::ExecutingEffects { effects };
}

pub fn update_executing_effects(game: &mut Game) {
    let effects = match &game.state {
        GameState::ExecutingEffects { effects } => effects.clone(),
        _ => return,
    };

    for effect in effects {
        execute_effect(game, effect);
    }
    
    to_ending_turn(game);
}

fn execute_effect(game: &mut Game, effect: Effect) -> Option<()> {
    match effect {
        Effect::Damage { min, max, target } => {
            let unit = game.unit_mut(target)?;
            let amount = roll(min, max);
            unit.hp = unit.hp.saturating_sub(amount);
        }
    }
    Some(())
}

fn roll(min: u16, max: u16) -> u16 {
    let roll1 = rand::gen_range(min, max + 1);
    let roll2 = rand::gen_range(min, max + 1);
    (roll1 + roll2) / 2
}
