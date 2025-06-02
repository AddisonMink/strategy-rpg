use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Animation {
    pub kind: AnimationKind,
    pub duration: f32,
    pub elapsed: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationKind {
    Number {
        coord: Coord,
        value: i32,
        color: Color,
    },
}
