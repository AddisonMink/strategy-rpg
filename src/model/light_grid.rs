use crate::coord::Coord;
use crate::entity::*;
use crate::map::Map;

/// A grid that holds light values for each coordinate in the map.
/// Each light value is a u16 representing the distance of that coordinate from edge of the nearest light source.
pub struct LightGrid {
    pub lights: Vec<u16>,
}

impl LightGrid {
    pub fn new(map: &Map, entities: &Entities) -> Self {
        let mut lights: Vec<u16> = vec![u16::max_value(); (Map::WIDTH * Map::HEIGHT) as usize];

        for light in entities.iter_lights() {
            if let Some(position) = entities.position(light.id) {
                let radius = light.radius;
                let center = position.coord;
                for x in 0..Map::WIDTH {
                    for y in 0..Map::HEIGHT {
                        let coord = Coord { x, y };
                        if map.check_line_of_sight(center, coord) {
                            let distance = center.manhattan_distance(coord).saturating_sub(radius);
                            let index = y as usize * Map::WIDTH as usize + x as usize;
                            lights[index] = lights[index].min(distance);
                        }
                    }
                }
            }
        }

        LightGrid { lights }
    }

    pub fn distance_from_light(&self, coord: Coord) -> u16 {
        let index = coord.y as usize * Map::WIDTH as usize + coord.x as usize;
        self.lights[index]
    }
}
