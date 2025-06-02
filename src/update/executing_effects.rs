use std::collections::VecDeque;

use crate::prelude::*;

use super::showing_animations::to_showing_animations;

const NUMBER_ANIMATION_DURATION: f32 = 0.5;

pub fn to_executing_effects(game: &mut Game, effects: VecDeque<Effect>) {
    game.state = GameState::ExecutingEffects { effects };
}

pub fn update_executing_effects(game: &mut Game) {
    let mut effects = match &game.state {
        GameState::ExecutingEffects { effects } => effects.clone(),
        _ => return,
    };

    let mut animations: VecDeque<Animation> = VecDeque::new();

    while let Some(effect) = effects.pop_front() {
        if let Some(new_animations) = execute_effect(game, effect.clone()) {
            animations.extend(new_animations);
            break;
        }
    }

    to_showing_animations(game, animations, effects);
}

fn execute_effect(game: &mut Game, effect: Effect) -> Option<VecDeque<Animation>> {
    let mut animations: VecDeque<Animation> = VecDeque::new();

    match effect {
        Effect::Damage { min, max, target } => {
            let unit = game.unit_mut(target)?;
            let amount = roll(min, max);
            unit.hp = unit.hp.saturating_sub(amount);
            animations.push_back(number_animation(unit.coord, -(amount as i32), RED));
        }
    }
    Some(animations)
}

fn roll(min: u16, max: u16) -> u16 {
    let roll1 = rand::gen_range(min, max + 1);
    let roll2 = rand::gen_range(min, max + 1);
    (roll1 + roll2) / 2
}

fn number_animation(coord: Coord, number: i32, color: Color) -> Animation {
    Animation {
        elapsed: 0.0,
        duration: NUMBER_ANIMATION_DURATION,
        kind: AnimationKind::Number {
            coord,
            value: number,
            color,
        },
    }
}
