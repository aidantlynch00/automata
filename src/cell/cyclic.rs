use macroquad::color::Color;
use macroquad::rand::gen_range;
use crate::cell::Cell;

pub struct Cyclic {
    value: usize,
    palette: &'static [Color],
    threshold: usize,
}

pub struct CyclicParams {
    pub palette: &'static [Color],
    pub threshold: usize,
}

impl Cell for Cyclic {
    type Params = CyclicParams;

    fn new(params: &Self::Params) -> Self {
        let bin: f32 = 1.0 / params.palette.len() as f32;
        let value = (gen_range(0.0, 1.0) / bin).floor() as usize;
        Cyclic {
            value,
            palette: params.palette,
            threshold: params.threshold,
        }
    }

    fn next<'a>(&'a self, neighbors: impl IntoIterator<Item = &'a Self>) -> Self {
        let next_value = (self.value + 1) % self.palette.len();
        let count = neighbors
            .into_iter()
            .map(|neighbor| if next_value == neighbor.value { 1 } else { 0 })
            .sum::<usize>();

        let value = if count >= self.threshold { next_value } else { self.value };
        Cyclic {
            value,
            palette: self.palette,
            threshold: self.threshold,
        }
    }

    fn color(&self) -> Color {
        self.palette[self.value]
    }
}
