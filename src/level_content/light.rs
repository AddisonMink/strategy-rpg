use crate::engine::*;
use crate::level_model::*;

pub fn add_point_light(level: &mut Level, coord: Coord, radius: u16, color: Color) {
    let entity = level.next_id;
    level.next_id.0 += 1;

    level.positions.insert(entity, Position::new(entity, coord));
    
    level
        .lights
        .insert(entity, Light::new(entity, radius, color));
}
