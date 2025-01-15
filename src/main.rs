mod automata;
mod cell;
mod time;

use std::process::exit;
use std::sync::LazyLock;
use macroquad::prelude::*;
use miniquad::conf::Platform;
use automata::Automata;
use cell::life::{Life, LifeParams};
use time::GenerationTimer;

static SCREEN_DIMS: LazyLock<(f32, f32)> = LazyLock::new(|| {
    (screen_width(), screen_height())
});

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
    const CELL_SIZE: f32 = 5.0;
    const GENS_PER_SEC: f32 = 10.0;
    for _ in 0..3 { next_frame().await }

    // set up automata
    let mut automata: Automata<Life> = Automata::new(
        CELL_SIZE,
        LifeParams {
            alive_ratio: 0.5,
        }
    );

    let mut timer = GenerationTimer::new(GENS_PER_SEC);
    loop {
        // handle key presses
        if is_key_pressed(KeyCode::Q) {
            exit(0);
        }
        
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::L) {
            timer.inc_rate();
        }

        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::H) {
            timer.dec_rate();
        }

        clear_background(BLACK);

        // render automata on screen
        automata.render();

        if timer.generate() {
            // calculate next generation of automata
            automata.next_gen();
        }

        next_frame().await;
    }
}
