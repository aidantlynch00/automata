use macroquad::prelude::*;
use macroquad::rand::gen_range;
use crate::cell::Cell;

#[derive(PartialEq, Eq)]
pub enum Brain {
    Dead,
    Dying,
    Alive
}

pub struct BrainParams {
    pub alive_ratio: f32,
}

impl Cell for Brain {
    type Params = BrainParams;

    fn new(params: &Self::Params) -> Self {
        if gen_range(0.0, 1.0) < params.alive_ratio {
            Brain::Alive
        }
        else {
            Brain::Dead
        }
    }

    fn next<'a>(&'a self, params: &Self::Params, neighbors: impl IntoIterator<Item = &'a Self>) -> Self {
        match *self {
            Brain::Dead => {
                let count = neighbors.into_iter()
                    .filter(|neighbor| **neighbor == Brain::Alive)
                    .count();

                match count {
                    2 => Brain::Alive,
                    _ => Brain::Dead
                }
            },
            Brain::Dying => Brain::Dead,
            Brain::Alive => Brain::Dying,
        }
    }

    fn color(&self, params: &Self::Params) -> Color {
        match *self {
            Brain::Dead => BLACK,
            Brain::Dying => BLUE,
            Brain::Alive => WHITE
        }
    }
}

impl Default for Brain {
    fn default() -> Self {
        Brain::Dead
    }
}
