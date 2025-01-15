mod automata;
mod cell;

use std::process::exit;
use std::sync::LazyLock;
use macroquad::prelude::*;
use miniquad::conf::Platform;
use automata::Automata;
use cell::life::{Life, RandomLifeParams};

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
    const CELL_SIZE: usize = 5;
    for _ in 0..3 { next_frame().await }

    // set up automata
    let mut automata: Automata<Life> = Automata::new(
        CELL_SIZE,
        RandomLifeParams {
            alive_ratio: 0.5,
        }
    );

    loop {
        // handle key presses
        if is_key_down(KeyCode::Q) {
            exit(0);
        }
        
        clear_background(BLACK);

        // render automata on screen
        automata.render();

        // calculate next generation of automata
        automata.next_gen();

        next_frame().await;
    }
}
