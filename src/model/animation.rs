use crate::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Animation {
    pub kind: AnimationKind,
    pub duration: f32,
    pub elapsed: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnimationKind {
    Number {
        coord: Coord,
        value: i32,
        color: Color,
    },
    Meter {
        coord: Coord,
        label: String,
        value: u16,
        max_value: u16,
        color: Color,
    },
    Message {
        coord: Coord,
        text: String,
        color: Color,
    },
}
