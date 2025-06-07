use crate::engine::*;

#[derive(Debug, Clone, Copy)]
pub struct Light {
    pub radius: u16,
    pub color: Color,
}

impl Light {
    pub fn new(radius: u16, color: Color) -> Self {
        Self { radius, color }
    }
}
