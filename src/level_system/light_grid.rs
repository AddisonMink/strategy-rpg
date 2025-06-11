use crate::engine::*;
use crate::level_model::*;

pub fn update_light_grid(level: &mut Level) {
    let mut lights: Vec<u16> = vec![u16::max_value(); (Map::WIDTH * Map::HEIGHT) as usize];
    let mut colors: Vec<Color> = vec![BLACK; (Map::WIDTH * Map::HEIGHT) as usize];

    let lights_iter = level
        .lights
        .values()
        .flat_map(|l| level.positions.get(&l.entity).map(|p| (p.coord, l)));

    for (center, light) in lights_iter {
        for x in 0..Map::WIDTH {
            for y in 0..Map::HEIGHT {
                let coord = Coord { x, y };
                let radius = light.radius as f32;

                if level.map.check_line_of_sight(center, coord) {
                    let distance = center
                        .manhattan_distance(coord)
                        .saturating_sub(light.radius);

                    let distance_from_radius = (distance as f32 - radius).max(0.0);
                    let light_alpha = (1.0 - distance_from_radius / radius).max(0.0);
                    let color = light.color.with_alpha(light_alpha);
                    let index = y as usize * Map::WIDTH as usize + x as usize;

                    lights[index] = lights[index].min(distance);
                    colors[index] = color::add_colors(colors[index], color);
                }
            }
        }
    }

    for color in colors.iter_mut() {
        *color = color::normalize_color(*color);
    }

    level.light_grid = LightGrid {
        distances_from_light: lights,
        light_colors: colors,
    }
}
