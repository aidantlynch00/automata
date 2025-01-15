use std::mem::MaybeUninit;
use macroquad::prelude::*;
use crate::SCREEN_DIMS;
use crate::cell::Cell;

pub struct Automata<C: Cell> {
    cell_size: usize,
    cols: usize,
    rows: usize,
    current: Box<[C]>,
    next: Box<[MaybeUninit<C>]>,
}

impl<C: Cell> Automata<C> {
    pub fn new(cell_size: usize, params: C::RandomParams) -> Automata<C> {
        // determine the grid size based on screen and cell size
        let (sw, sh) = *SCREEN_DIMS;
        let cols = sw as usize / cell_size;
        let rows = sh as usize / cell_size;

        let total = cols * rows;
        let (current, next): (Vec<C>, Vec<MaybeUninit<C>>) = (0..total)
            .map(|_| (C::random(&params), MaybeUninit::uninit()))
            .unzip();

        Automata {
            cell_size,
            cols,
            rows,
            current: current.into_boxed_slice(),
            next: next.into_boxed_slice(),
        }
    }

    pub fn next_gen(&mut self) {
        for col in 0..self.cols {
            for row in 0..self.rows {

            }
        }
    }

    pub fn render(&self) {
        for (index, cell) in self.current.iter().enumerate() {
            let (x, y) = self.linear_to_grid(index);
            let color = cell.color();

            draw_rectangle(
                (x * self.cell_size) as f32,
                (y * self.cell_size) as f32,
                self.cell_size as f32,
                self.cell_size as f32,
                color
            );
        }
    }

    fn linear_to_grid(&self, index: usize) -> (usize, usize) {
        let col = index / self.rows;
        let row = index % self.rows;
        (col, row)
    }

    fn grid_to_linear(&self, col: usize, row: usize) -> usize {
        col * self.rows + row
    }
}
