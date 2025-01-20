use macroquad::prelude::*;
use itertools::iproduct;
use crate::SCREEN_DIMS;
use crate::cell::Cell;

pub trait AutomataTrait {
    fn next(&mut self);
    fn render(&self);
}

pub struct Automata<C: Cell> {
    cell_size: f32,
    cols: usize,
    rows: usize,
    params: C::Params,
    current: Box<[C]>,
    next: Box<[C]>,
}

impl<C: Cell> Automata<C> {
    pub fn new(cell_size: f32, params: C::Params) -> Automata<C> {
        // determine the grid size based on screen and cell size
        let (sw, sh) = *SCREEN_DIMS;
        let cols = (sw / cell_size) as usize;
        let rows = (sh / cell_size) as usize;

        let total = cols * rows;
        let (current, next): (Vec<C>, Vec<C>) = (0..total)
            .map(|_| (C::new(&params), C::default()))
            .unzip();

        Automata {
            cell_size,
            cols,
            rows,
            params,
            current: current.into_boxed_slice(),
            next: next.into_boxed_slice(),
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

impl<C: Cell> AutomataTrait for Automata<C> {
    fn next(&mut self) {
        let diff = [-1, 0, 1];
        let mut neighbors: [Option<&C>; 8] = [None; 8];

        for col in 0..self.cols as isize {
            for row in 0..self.rows as isize {
                // push references to this cell's neighbors
                let mut ref_index: usize = 0;
                let diffs = iproduct!(diff.iter().cloned(), diff.iter().cloned());
                for (dc, dr) in diffs {
                    // skip the current cell
                    if dc == 0 && dr == 0 {
                        continue;
                    }

                    let ncol = col + dc;
                    let nrow = row + dr;

                    // skip if the neighbor is off the grid
                    if ncol < 0 || ncol >= self.cols as isize || nrow < 0 || nrow >= self.rows as isize {
                        continue;
                    }

                    // push reference to neighbor
                    let nindex = self.grid_to_linear(ncol as usize, nrow as usize);
                    neighbors[ref_index] = Some(&self.current[nindex]);
                    ref_index += 1;
                }

                // take all references in the neighbors array
                let neighbors_iter = neighbors.iter_mut()
                    .flat_map(|neighbor_opt| neighbor_opt.take());

                // calculate the next cell
                let index = self.grid_to_linear(col as usize, row as usize);
                self.next[index] = self.current[index].next(&self.params, neighbors_iter);
            }
        }

        std::mem::swap(&mut self.current, &mut self.next);
    }

    fn render(&self) {
        for (index, cell) in self.current.iter().enumerate() {
            let (x, y) = self.linear_to_grid(index);
            let color = cell.color(&self.params);

            draw_rectangle(
                x as f32 * self.cell_size,
                y as f32 * self.cell_size,
                self.cell_size,
                self.cell_size,
                color
            );
        }
    }
}
