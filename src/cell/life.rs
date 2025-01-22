use macroquad::prelude::*;
use macroquad::rand::gen_range;
use crate::cell::Cell;

#[derive(PartialEq, Eq)]
pub enum Life {
    Dead,
    Alive,
}

pub struct LifeParams {
    pub alive_ratio: f32,
}

impl Cell for Life {
    type Params = LifeParams;

    fn new(params: &Self::Params) -> Self {
        if gen_range(0.0, 1.0) < params.alive_ratio {
            Life::Alive
        }
        else {
            Life::Dead
        }
    }

    fn next<'a>(&'a self, _params: &Self::Params, neighbors: impl IntoIterator<Item = &'a Self>) -> Self
    {
        let count = neighbors.into_iter()
            .filter(|neighbor| **neighbor == Life::Alive)
            .count();

        match *self {
            Life::Dead => match count {
                3 => Life::Alive,
                _ => Life::Dead,
            },
            Life::Alive => match count {
                2 | 3 => Life::Alive,
                _ => Life::Dead,
            },
        }
    }

    fn color(&self, _params: &Self::Params) -> Color {
        match *self {
            Life::Dead => WHITE,
            Life::Alive => BLACK,
        }
    }
}

impl Default for Life {
    fn default() -> Self {
        Life::Dead
    }
}
