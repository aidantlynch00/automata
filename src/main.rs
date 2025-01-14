use std::process::exit;
use macroquad::prelude::*;
use miniquad::conf::Platform;

fn window_conf() -> Conf {
    Conf {
        window_title: "Automata".to_string(),
        fullscreen: true,
        window_resizable: false,
        platform: Platform {
            swap_interval: Some(0),
            ..Platform::default()
        },
        ..Conf::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    const CELL_SIZE: u16 = 50;
    for _ in 0..3 { next_frame().await }

    // get screen dimensions
    let sw = screen_width();
    let sh = screen_height();

    // determine the grid size based on screen and cell size
    let grid_w = sw as u16 / CELL_SIZE;
    let grid_h = sh as u16 / CELL_SIZE;

    let mut image = Image::gen_image_color(grid_w, grid_h, WHITE);
    let texture = Texture2D::from_image(&image);
    let texture_params = DrawTextureParams {
        dest_size: Some(Vec2::new(sw, sh)),
        ..DrawTextureParams::default()
    };

    loop {
        // handle key presses
        if is_key_down(KeyCode::Q) {
            exit(0);
        }

        clear_background(WHITE);

        // update texture with image data and render
        texture.update(&image);
        draw_texture_ex(&texture, 0.0, 0.0, WHITE, texture_params.clone());

        next_frame().await;
    }
}
