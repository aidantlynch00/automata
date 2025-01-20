use macroquad::color::Color;
use macroquad::rand::gen_range;
use crate::cell::Cell;

pub mod palette {
    use std::sync::LazyLock;
    use macroquad::prelude::*;

    pub static COLORS: &'static [Color] = &[PINK, RED, ORANGE, YELLOW, GREEN, BLUE, VIOLET, MAGENTA];
    pub static GRAYSCALE: LazyLock<Vec<Color>> = LazyLock::new(|| {
        (1..=18)
            .into_iter()
            .map(|i| {
                let val = 1.0 / i as f32;
                Color::new(val, val, val, 1.0)
            })
            .collect()
    });
}

pub struct Cyclic {
    value: usize,
}

pub struct CyclicParams {
    pub palette: &'static [Color],
    pub threshold: u8,
}

impl Cell for Cyclic {
    type Params = CyclicParams;

    fn new(params: &Self::Params) -> Self {
        let bin: f32 = 1.0 / params.palette.len() as f32;
        let value = (gen_range(0.0, 1.0) / bin).floor() as usize;
        Cyclic {
            value,
        }
    }

    fn next<'a>(&'a self, params: &Self::Params, neighbors: impl IntoIterator<Item = &'a Self>) -> Self {
        let next_value = (self.value + 1) % params.palette.len();
        let count = neighbors
            .into_iter()
            .map(|neighbor| if next_value == neighbor.value { 1 } else { 0 })
            .sum::<u8>();

        let value = if count >= params.threshold { next_value } else { self.value };
        Cyclic {
            value,
        }
    }

    fn color(&self, params: &Self::Params) -> Color {
        params.palette[self.value]
    }
}
