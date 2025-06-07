use super::ending_turn;
use super::model::*;
use crate::engine::*;
use std::collections::VecDeque;

pub fn transition(battle: &mut Battle, effects: VecDeque<Effect>) {
    battle.state = BattleState::ExecutingEffects {
        effects,
        animations: VecDeque::new(),
    };
}

pub fn update(battle: &mut Battle, delta_time: f32) {
    let (effects, animations) = unpack(battle);

    // No effects or animations left, end turn.
    if effects.is_empty() && animations.is_empty() {
        ending_turn::transition(battle);
    }
    // Handle animations.
    else if let Some(animation) = animations.front_mut() {
        animation.timer.update(delta_time);
        if animation.timer.is_finished() {
            animations.pop_front();
        }
    // Handle effects.
    } else if let Some(effect) = effects.pop_front() {
        match effect {
            Effect::Damage { min, max, target } => exec_damage(battle, min, max, target),
            Effect::QueueAnimation { animation } => {
                animations.push_back(animation);
                Some(())
            }
        };
    }
}

fn exec_damage(battle: &mut Battle, min: u16, max: u16, target: UnitId) -> Option<()> {
    let unit = battle.unit_mut(target)?;
    let coord = unit.coord;
    let damage = roll(min, max);
    unit.hp = unit.hp.saturating_sub(damage);

    let (_, animations) = unpack(battle);
    animations.push_back(Animation::number(coord, -(damage as i32), RED));
    Some(())
}

fn unpack(game: &mut Battle) -> (&mut VecDeque<Effect>, &mut VecDeque<Animation>) {
    let BattleState::ExecutingEffects {
        effects,
        animations,
    } = &mut game.state
    else {
        panic!("Battle state is not ExecutingEffects");
    };
    (effects, animations)
}

fn roll(min: u16, max: u16) -> u16 {
    let roll1 = gen_range(min, max + 1);
    let roll2 = gen_range(min, max + 1);
    (roll1 + roll2) / 2
}
