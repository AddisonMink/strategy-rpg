use std::vec;

use super::Action;
use super::Battle;
use super::Effect;
use crate::engine::*;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UnitId(pub u16);

#[derive(Debug, Clone, PartialEq)]
pub struct Unit {
    // Identifiers
    pub id: UnitId,
    pub name: ShortString,
    pub glyph: Glyph,
    pub side: Side,
    // Attributes
    pub movement: u16,
    pub hp_max: u16,
    pub vision: u16,
    // State
    pub coord: Coord,
    pub hp: u16,
    select_move: Option<fn(&Battle, &Unit) -> Option<VecDeque<Coord>>>,
    select_action: Option<fn(&Battle, &Unit) -> Option<VecDeque<Effect>>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Player,
    NPC,
}

pub struct UnitData {
    pub name: ShortString,
    pub glyph: Glyph,
    pub side: Side,
    pub movement: u16,
    pub vision: u16,
    pub hp_max: u16,
}

impl Unit {
    pub fn new(id: UnitId, coord: Coord, data: UnitData) -> Self {
        Self {
            id,
            name: data.name,
            glyph: data.glyph,
            side: data.side,
            movement: data.movement,
            vision: data.vision,
            hp_max: data.hp_max,
            coord,
            hp: data.hp_max,
            select_move: None,
            select_action: None,
        }
    }

    pub fn new_npc(
        id: UnitId,
        coord: Coord,
        data: UnitData,
        select_move: fn(&Battle, &Unit) -> Option<VecDeque<Coord>>,
        select_action: fn(&Battle, &Unit) -> Option<VecDeque<Effect>>,
    ) -> Self {
        Self {
            id,
            name: data.name,
            glyph: data.glyph,
            side: data.side,
            movement: data.movement,
            vision: data.vision,
            hp_max: data.hp_max,
            coord,
            hp: data.hp_max,
            select_move: Some(select_move),
            select_action: Some(select_action),
        }
    }

    pub fn actions(&self) -> Vec<Action> {
        vec![Action::ATTACK]
    }

    pub fn npc_select_move(&self, battle: &Battle) -> Option<VecDeque<Coord>> {
        self.select_move.and_then(|f| f(battle, self))
    }

    pub fn npc_select_action(&self, battle: &Battle) -> Option<VecDeque<Effect>> {
        self.select_action.and_then(|f| f(battle, self))
    }
}
