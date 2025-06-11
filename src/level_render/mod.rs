use crate::engine::color::mix_color;
use crate::engine::*;
use crate::level_model::*;

const INFO_PANEL_ORIGIN: (f32, f32) = (360.0, 10.0);

pub fn render_level(level: &Level) {
    draw_map(level);
    draw_state(level);
}

fn draw_map(level: &Level) {
    grid::draw_frame("MAP");
    for y in 0..Map::HEIGHT {
        for x in 0..Map::WIDTH {
            let coord = Coord::new(x, y);
            if !level.player_vision.tile_visible(coord) {
                continue;
            }

            let distance_from_light = level.light_grid.distance_from_light(coord);
            let tile = level.map.tile(coord);
            let unoccupied = level.unit_at(coord).is_none();

            let light_color = if distance_from_light == 0 {
                level.light_grid.light_color(coord)
            } else {
                BLACK
            };

            if let Some(color) = tile.bg_color {
                grid::draw_square(coord, mix_color(color, light_color, 0.5));
            }

            if unoccupied {
                let glyph_color = mix_color(tile.glyph.color, light_color, 0.5);
                let glyph = Glyph::new(tile.glyph.symbol, glyph_color);
                grid::draw_glyph(coord, glyph);
            }
        }
    }

    level
        .positions
        .values()
        .filter(|p| level.player_vision.tile_visible(p.coord))
        .filter_map(|p| level.units.get(&p.entity).map(|unit| (p, unit)))
        .for_each(|(pos, unit)| {
            let distance_from_light = level.light_grid.distance_from_light(pos.coord);
            let light_color = if distance_from_light == 0 {
                level.light_grid.light_color(pos.coord)
            } else {
                BLACK
            };

            let glyph_color = mix_color(unit.glyph.color, light_color, 0.5);
            let glyph = Glyph::new(unit.glyph.symbol, glyph_color);
            grid::draw_glyph(pos.coord, glyph);
        });
}

fn draw_state(level: &Level) {
    match &level.state {
        LevelState::SelectingMove {
            valid_moves,
            path,
            action_previews,
        } => {
            for c in valid_moves.iter() {
                grid::draw_square(*c, WHITE.with_alpha(0.5));
            }
            for c in path.iter().flatten() {
                grid::draw_glyph(*c, Glyph::new('o', WHITE));
            }

            let mut panel = Panel::builder("ACTIONS", WHITE);
            for action in action_previews {
                let color = if action.valid { WHITE } else { GRAY };
                panel = panel.line(action.name.to_string(), color);
            }
            panel.build().draw(INFO_PANEL_ORIGIN.0, INFO_PANEL_ORIGIN.1);
        }
        LevelState::SelectingAction {
            panel,
            panel_origin,
            target_coords,
            ..
        } => {
            panel.draw(panel_origin.0, panel_origin.1);

            for coord in target_coords.iter().flatten() {
                grid::draw_square(*coord, WHITE.with_alpha(0.5));
            }
        }
        _ => {}
    }
}
