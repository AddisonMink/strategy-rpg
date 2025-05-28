use super::light::Light;
use crate::util::Coord;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PointLightId(pub u16);

#[derive(Debug, Clone, Copy)]
pub struct PointLight {
    pub id: PointLightId,
    pub coord: Coord,
    pub light: Light,
}
