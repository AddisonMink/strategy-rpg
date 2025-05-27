use crate::coord::Coord;
use crate::entity::*;
use crate::map::Map;

/// A grid that holds light values for each coordinate in the map.
/// Each light value is a u16 representing the distance of that coordinate from edge of the nearest light source.
pub struct LightGrid {
    pub lights: Vec<u16>,
}

impl LightGrid {
    pub fn new(entities: &Entities) -> Self {
        let mut lights: Vec<u16> = vec![u16::max_value(); (Map::WIDTH * Map::HEIGHT) as usize];

        for light in entities.iter_lights() {
            if let Some(position) = entities.get_position(light.id) {
                let radius = light.radius;
                let center = position.coord;
                let x0 = center.x.saturating_sub(radius);
                let x1 = (center.x + radius).min(Map::WIDTH - 1);
                let y0 = center.y.saturating_sub(radius);
                let y1 = (center.y + radius).min(Map::HEIGHT - 1);

                for x in x0..=x1 {
                    for y in y0..=y1 {
                        let coord = Coord { x, y };
                        if coord.manhattan_distance(&center) <= radius {
                            let index = y as usize * Map::WIDTH as usize + x as usize;
                            lights[index] = 0;
                        }
                    }
                }
            }
        }

        LightGrid { lights }
    }

    pub fn light_value(&self, coord: Coord) -> u16 {
        let index = coord.y as usize * Map::WIDTH as usize + coord.x as usize;
        self.lights[index]
    }
}
