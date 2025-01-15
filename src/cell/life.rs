use macroquad::prelude::*;
use macroquad::rand::gen_range;
use crate::cell::Cell;

pub enum Life {
    Alive,
    Dead,
}

pub struct RandomLifeParams {
    pub alive_ratio: f32,
}

impl Cell for Life {
    type RandomParams = RandomLifeParams;

    fn random(params: &Self::RandomParams) -> Self {
        if gen_range(0.0, 1.0) < params.alive_ratio {
            Life::Alive
        }
        else {
            Life::Dead
        }
    }

    fn next<'a>(&'a self, neighbors: impl Iterator<Item = &'a Self>) -> Self {
        Life::Alive
    }

    fn color(&self) -> Color {
        match *self {
            Life::Alive => BLACK,
            Life::Dead => WHITE,
        }
    }
}
