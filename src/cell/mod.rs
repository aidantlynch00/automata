pub mod life;
pub mod cyclic;
pub mod brain;
pub mod prelude;

use macroquad::color::Color;

pub trait Cell {
    type Params;
    fn new(params: &Self::Params) -> Self;
    fn next<'a>(&'a self, params: &Self::Params, neighbors: impl IntoIterator<Item = &'a Self>) -> Self;
    fn color(&self, params: &Self::Params) -> Color;
}
