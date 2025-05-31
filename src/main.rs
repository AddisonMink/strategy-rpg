use macroquad::prelude::*;

mod data;
mod model;
mod prelude;
mod render;
mod update;
mod util;

use prelude::*;

#[macroquad::main("Strategy RPG")]
async fn main() {
    asset::load_assets().await;

    let mut game = Game::new(Map::new());
    let player_id = game.add_unit(Coord::new(14, 1), data::unit::make_player);
    let player = game.unit_mut(player_id).unwrap();
    player.light = Some(Light::new(5, ORANGE));
    game.add_unit(Coord::new(1, 1), data::unit::make_goon);
    game.add_point_light(make_point_light);
    game.light_grid = LightGrid::new(&game);
    game.next_turn();
    let mut time = 0.0;

    loop {
        let delta_time = get_frame_time();
        time += delta_time;
        let flicker = algorithm::perlin_noise_1d(time, 0.5, 1.0, 42);
        update::update_game(&mut game, delta_time);
        clear_background(BLACK);
        render::draw_game(&game, flicker);
        draw_text("0.0.3", 10.0, 274.0, 16.0, WHITE);
        next_frame().await;
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
