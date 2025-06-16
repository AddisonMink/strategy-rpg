use crate::engine::*;
use crate::level_model::*;

pub fn add_point_light(level: &mut Level, coord: Coord, radius: u16, color: Color) {
    let id = level.next_point_light_id;
    level.next_point_light_id.0 += 1;

    level
        .point_lights
        .insert(id, PointLight::new(id, coord, Light { radius, color }));
}
