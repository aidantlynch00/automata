pub mod life;

use macroquad::color::Color;

pub trait Cell {
    type RandomParams;
    fn random(params: &Self::RandomParams) -> Self;
    fn next<'a>(&'a self, neighbors: impl Iterator<Item = &'a Self>) -> Self;
    fn color(&self) -> Color;
}
