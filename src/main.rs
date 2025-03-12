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
use automata::{Automata, AutomataParams, AutomataTrait};
use cell::prelude::*;
use time::Ticker;

static SCREEN_DIMS: LazyLock<(f32, f32)> = LazyLock::new(|| {
    (screen_width(), screen_height())
});

static SAMPLES_PER_SEC: u32 = 10;

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
    let params = AutomataParams {
        cell_size: args.cell_size,
        threads: args.threads,
        chunks: args.chunks,
    };

    let mut automata: Box<dyn AutomataTrait> = match args.cell {
        CellType::Life(life_params) => {
            Box::new(Automata::<Life>::new(
                params,
                LifeParams {
                    alive_ratio: life_params.percent_arg.percentage as f32 / 100.0,
                    rule: life_params.rule,
                }
            ))
        },
        CellType::Cyclic(cyclic_params) => {
            let palette = match cyclic_params.palette {
                Palette::Rainbow => &*COLORS,
                Palette::Grayscale => &*GRAYSCALE,
            };

            Box::new(Automata::<Cyclic>::new(
                params,
                CyclicParams {
                    threshold: cyclic_params.threshold,
                    palette,
                }
            ))
        },
        CellType::Brain(brain_params) => {
            Box::new(Automata::<Brain>::new(
                params,
                BrainParams {
                    alive_ratio: brain_params.percentage as f32 / 100.0,
                }
            ))
        },
    };

    clear_background(BLACK);

    // enter main loop
    let mut sample_timer = Ticker::new(SAMPLES_PER_SEC);
    let mut render_timer = Ticker::new(args.gens_per_sec);
    loop {
        if sample_timer.tick() {
            // handle key presses
            if is_key_down(KeyCode::Q) {
                exit(0);
            }
            
            let (_wheel_x, wheel_y) = mouse_wheel();
            match wheel_y {
                -1.0 => render_timer.dec_rate(),
                 1.0 => render_timer.inc_rate(),
                _ => {}
            }
        }

        if render_timer.tick() {
            // calculate next generation of automata
            automata.next();

            // render automata on screen
            automata.render();
            next_frame().await;
        }
    }
}
