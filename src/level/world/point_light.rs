use super::Light;
use crate::engine::*;
use crate::util::Coord;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PointLightId(pub u32);

#[derive(Debug, Clone, Copy)]
pub struct PointLight {
    pub id: PointLightId,
    pub light: Light,
    pub coord: Coord,
}

impl PointLight {
    pub fn new(id: PointLightId, radius: u16, color: Color, coord: Coord) -> Self {
        Self {
            id,
            light: Light { radius, color },
            coord,
        }
    }
}
