use std::collections::VecDeque;

use crate::prelude::*;

use super::showing_animations::to_showing_animations;

const NUMBER_ANIMATION_DURATION: f32 = 0.5;
const METER_ANIMATION_DURATION: f32 = 0.5;
const MESSAGE_ANIMATION_DURATION: f32 = 1.0;

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
        if let Some((new_effects, new_animations)) = execute_effect(game, effect.clone()) {
            effects.extend(new_effects);
            animations.extend(new_animations);
            if animations.len() > 0 {
                break;
            }
        }
    }

    to_showing_animations(game, animations, effects);
}

fn execute_effect(
    game: &mut Game,
    effect: Effect,
) -> Option<(VecDeque<Effect>, VecDeque<Animation>)> {
    let mut effects: VecDeque<Effect> = VecDeque::new();
    let mut animations: VecDeque<Animation> = VecDeque::new();

    match effect {
        Effect::Damage { min, max, target } => {
            let unit = game.unit_mut(target)?;
            let amount = roll(min, max);
            unit.hp = unit.hp.saturating_sub(amount);
            animations.push_back(number_animation(unit.coord, -(amount as i32), RED));
            animations.push_back(hp_meter_animation(unit));
            if unit.hp == 0 {
                animations.push_back(death_message_animation(unit.coord));
                effects.push_back(Effect::Kill { target });
            }
        }
        Effect::Kill { target } => {
            game.remove_unit(target);
        }
        Effect::QueueAnimation { animation } => {
            animations.push_back(animation);
        }
    }
    Some((effects, animations))
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

fn hp_meter_animation(unit: &Unit) -> Animation {
    Animation {
        elapsed: 0.0,
        duration: METER_ANIMATION_DURATION,
        kind: AnimationKind::Meter {
            coord: unit.coord,
            label: unit.name.clone(),
            value: unit.hp,
            max_value: unit.hp_max,
            color: RED,
        },
    }
}

fn death_message_animation(coord: Coord) -> Animation {
    Animation {
        elapsed: 0.0,
        duration: MESSAGE_ANIMATION_DURATION,
        kind: AnimationKind::Message {
            coord,
            text: "DEATH".to_string(),
            color: LIGHTGRAY,
        },
    }
}
