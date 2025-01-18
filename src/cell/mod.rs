pub mod life;
pub mod cyclic;

use macroquad::color::Color;

pub trait Cell {
    type Params;
    fn new(params: &Self::Params) -> Self;
    fn next<'a>(&'a self, neighbors: impl IntoIterator<Item = &'a Self>) -> Self;
    fn color(&self) -> Color;
}
