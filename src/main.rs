use macroquad::prelude::*;

mod model;
mod render;
mod update;
mod util;

use model::*;

#[macroquad::main("Strategy RPG")]
async fn main() {
    asset::load_assets().await;

    let mut game = Game::new(Map::new());
    game.add_unit(make_unit);
    game.add_unit(make_unit_2);
    game.add_point_light(make_point_light);
    game.light_grid = LightGrid::new(&game);
    game.next_turn();
    let mut time = 0.0;

    loop {
        time += get_frame_time();
        let flicker = algorithm::perlin_noise_1d(time, 0.5, 1.0, 42);

        update::update_game(&mut game);
        clear_background(BLACK);
        render::draw_map(&game, flicker);
        draw_text("0.0.1", 550.0, 16.0, 16.0, WHITE);
        next_frame().await;
    }
}

fn make_unit(id: UnitId) -> Unit {
    Unit {
        id,
        coord: Coord { x: 14, y: 1 },
        glyph: Glyph {
            symbol: '@',
            color: WHITE,
        },
        vision: 2,
        movement: 3,
        light: Some(Light {
            radius: 3,
            color: ORANGE,
        }),
    }
}

fn make_unit_2(id: UnitId) -> Unit {
    Unit {
        id,
        coord: Coord { x: 1, y: 1 },
        glyph: Glyph {
            symbol: 'A',
            color: WHITE,
        },
        vision: 2,
        movement: 3,
        light: None,
    }
}

fn make_point_light(id: PointLightId) -> PointLight {
    PointLight {
        id,
        coord: Coord { x: 1, y: 1 },
        light: Light {
            radius: 5,
            color: BLUE,
        },
    }
}
