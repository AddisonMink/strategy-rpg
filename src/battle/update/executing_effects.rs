use super::model::*;
use super::selecting_move;
use crate::engine::*;
use std::collections::VecDeque;

pub fn transition(battle: &mut Battle, effects: VecDeque<Effect>) {
    battle.state = BattleState::ExecutingEffects { effects };
}

pub fn update(battle: &mut Battle) {
    let BattleState::ExecutingEffects { effects } = &mut battle.state else {
        return;
    };

    if let Some(effect) = effects.pop_front() {
        let _ = match effect {
            Effect::Damage { min, max, target } => exec_damage(battle, min, max, target),
        };
    } else {
        selecting_move::transition(battle);
    }
}

fn exec_damage(battle: &mut Battle, min: u16, max: u16, target: UnitId) -> Option<()> {
    let unit = battle.unit_mut(target)?;
    let damage = roll(min, max);
    unit.hp = unit.hp.saturating_sub(damage);
    Some(())
}

fn roll(min: u16, max: u16) -> u16 {
    let roll1 = gen_range(min, max + 1);
    let roll2 = gen_range(min, max + 1);
    (roll1 + roll2) / 2
}
