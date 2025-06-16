use super::light::*;
use crate::engine::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PointLightId(pub u16);

#[derive(Debug, Clone, Copy)]
pub struct PointLight {
    pub id: PointLightId,
    pub coord: Coord,
    pub light: Light,
}

impl PointLight {
    pub fn new(id: PointLightId, coord: Coord, light: Light) -> Self {
        Self { id, coord, light }
    }
}
