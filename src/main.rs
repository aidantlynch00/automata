mod args;
mod automata;
mod cell;
mod time;

use std::process::exit;
use std::sync::LazyLock;
use std::time::SystemTime;
use macroquad::prelude::*;
use macroquad::rand::srand;
use miniquad::conf::Platform;
use clap::Parser;
use args::*;
use automata::{Automata, AutomataTrait};
use cell::life::{Life, LifeParams};
use cell::cyclic::{Cyclic, CyclicParams};
use cell::cyclic::palette::*;
use cell::brain::{Brain, BrainParams};
use time::GenerationTimer;

static SCREEN_DIMS: LazyLock<(f32, f32)> = LazyLock::new(|| {
    (screen_width(), screen_height())
});


fn window_conf() -> Conf {
    Conf {
        window_title: "Automata".to_string(),
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
    let args = AutomataArgs::parse();

    // set the screen size
    if args.window.fullscreen {
        set_fullscreen(true);
    }
    else {
        let width = args.window.width.unwrap();
        let height = args.window.height.unwrap();
        request_new_screen_size(width, height);
    }

    // This works around an issue where the true screen size is not available
    // for the first few frames.
    for _ in 0..3 { next_frame().await }

    // set a random seed so that each run is different
    srand(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs());

    // set up automata
    let mut automata: Box<dyn AutomataTrait> = match args.cell {
        CellType::Life(params) => {
            Box::new(Automata::<Life>::new(
                args.cell_size,
                LifeParams {
                    alive_ratio: params.percentage as f32 / 100.0,
                }
            ))
        },
        CellType::Cyclic(params) => {
            let palette = match params.palette {
                Palette::Rainbow => &*COLORS,
                Palette::Grayscale => &*GRAYSCALE,
            };

            Box::new(Automata::<Cyclic>::new(
                args.cell_size,
                CyclicParams {
                    threshold: params.threshold,
                    palette,
                }
            ))
        },
        CellType::Brain(params) => {
            Box::new(Automata::<Brain>::new(
                args.cell_size,
                BrainParams {
                    alive_ratio: params.percentage as f32 / 100.0,
                }
            ))
        },
    };

    let mut timer = GenerationTimer::new(args.gens_per_sec);
    loop {
        // handle key presses
        if is_key_pressed(KeyCode::Q) {
            exit(0);
        }
        
        let (_wheel_x, wheel_y) = mouse_wheel();
        match wheel_y {
            -1.0 => timer.dec_rate(),
             1.0 => timer.inc_rate(),
            _ => {}
        }

        clear_background(BLACK);

        // render automata on screen
        automata.render();

        if timer.generate() {
            // calculate next generation of automata
            automata.next();
        }

        next_frame().await;
    }
}
