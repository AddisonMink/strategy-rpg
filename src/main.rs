use macroquad::prelude::*;

#[macroquad::main("Strategy RPG")]
async fn main() {
    // Load the font (make sure the path is correct)
    let font = load_ttf_font("assets/fonts/kongtext.ttf").await.unwrap();
    let texture = load_texture("assets/textures/nineslice.png").await.unwrap();

    loop {
        clear_background(RED);

        draw_texture(&texture, 50.0, 50.0, WHITE);
        trace!("Texture loaded successfully");

        
        draw_text_ex(
            "Hello, Macroquad!",
            20.0,
            40.0,
            TextParams {
                font: Some(&font),
                font_size: 40,
                color: WHITE,
                ..Default::default()
            },
        );

        next_frame().await;
    }
}
