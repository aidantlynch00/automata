use std::thread;
use std::sync::Arc;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::ops::Range;
use macroquad::prelude::*;
use crate::SCREEN_DIMS;
use crate::cell::Cell;

pub trait AutomataTrait {
    fn next(&mut self);
    fn render(&self);
}

#[derive(Clone, Copy)]
struct GridSize {
    pub cols: usize,
    pub rows: usize,
}

struct WorkerItem<C>
where C: 'static + Cell + Send + Sync + Default + PartialEq + Eq,
      C::Params: 'static + Send + Sync
{
    pub current: Arc<Box<[C]>>,
    pub range: Range<usize>,
    pub result_send: Sender<(usize, C)>,
}

struct AutomataTexture {
    image: Image,
    texture: Texture2D,
    params: DrawTextureParams,
    pub first_update: bool,
}

pub struct AutomataParams {
    pub cell_size: f32,
    pub threads: usize,
    pub chunks: usize,
}

pub struct Automata<C>
where C: 'static + Cell + Send + Sync + Default + PartialEq + Eq,
      C::Params: 'static + Send + Sync
{
    grid_size: GridSize,
    cell_params: Arc<C::Params>,
    current: Arc<Box<[C]>>,
    next: Box<[C]>,
    chunks: Box<[Range<usize>]>,
    workers: Box<[thread::JoinHandle<()>]>,
    item_senders: Box<[Sender<WorkerItem<C>>]>,
    texture: AutomataTexture,
}

impl<C> Automata<C>
where C: 'static + Cell + Send + Sync + Default + PartialEq + Eq,
      C::Params: 'static + Send + Sync
{
    pub fn new(params: AutomataParams, cell_params: C::Params) -> Automata<C> {
        let AutomataParams { cell_size, threads, chunks } = params;

        // determine the grid size based on screen and cell size
        let (sw, sh) = *SCREEN_DIMS;
        let cols = (sw / cell_size) as usize;
        let rows = (sh / cell_size) as usize;

        let total = cols * rows;
        let (current, next): (Vec<C>, Vec<C>) = (0..total)
            .map(|_| (C::new(&cell_params), C::default()))
            .unzip();

        let chunk_size = total / chunks;
        let mut chunks_vec: Vec<Range<usize>> = (0..chunks - 1)
            .map(|n| {
                let start = chunk_size * n;
                start..(start + chunk_size)
            })
            .collect();
        chunks_vec.push((chunk_size * (chunks - 1))..total);

        let grid_size = GridSize { cols, rows };
        let cell_params = Arc::new(cell_params);
        let (workers, senders): (Vec<thread::JoinHandle<()>>, Vec<Sender<WorkerItem<C>>>) = (0..threads)
            .map(|_| {
                let cell_params_clone = Arc::clone(&cell_params);
                let (item_send, item_recv) = channel();
                let handle = thread::spawn(move || {
                    Automata::calculate_chunk(grid_size, cell_params_clone, item_recv);
                });

                (handle, item_send)
            })
            .unzip();

        let image = Image::gen_image_color(cols as u16, rows as u16, BLACK);
        let texture = Texture2D::from_image(&image);
        texture.set_filter(FilterMode::Nearest);

        Automata {
            grid_size,
            cell_params,
            current: Arc::new(current.into_boxed_slice()),
            next: next.into_boxed_slice(),
            chunks: chunks_vec.into_boxed_slice(),
            workers: workers.into_boxed_slice(),
            item_senders: senders.into_boxed_slice(),
            texture: AutomataTexture {
                image,
                texture,
                params: DrawTextureParams {
                    dest_size: Some((*SCREEN_DIMS).into()),
                    ..DrawTextureParams::default()
                },
                first_update: true,
            },
        }
    }

    fn calculate_chunk(
        grid_size: GridSize,
        cell_params: Arc<C::Params>,
        item_recv: Receiver<WorkerItem<C>>
    ) {
        static DIFFS: [(isize, isize); 8] = [
            (-1, -1), (0, -1), (1, -1),
            (-1,  0),          (1,  0),
            (-1,  1), (0,  1), (1,  1)
        ];

        let GridSize { cols, rows } = grid_size;
        while let Ok(WorkerItem { current, range, result_send }) = item_recv.recv() {
            for index in range.into_iter() {
                let mut neighbors = [None; 8];
                let neighbor_diff_iter = DIFFS.iter()
                    .zip(neighbors.iter_mut());

                let (col, row) = linear_to_grid(rows, index);
                for ((dcol, drow), nopt) in neighbor_diff_iter {
                    let ncol = col as isize + dcol;
                    let nrow = row as isize + drow;

                    *nopt = if ncol < 0 || ncol >= cols as isize || nrow < 0 || nrow >= rows as isize {
                        None
                    }
                    else {
                        let nindex = grid_to_linear(rows, ncol as usize, nrow as usize);
                        Some(&current[nindex])
                    };
                }

                // calculate next cell
                let neighbors_iter = neighbors.iter_mut()
                    .flat_map(|nopt| nopt.take());
                let next = current[index].next(&cell_params, neighbors_iter);

                // SAFETY: receiver is only dropped when all senders are dropped
                result_send.send((index, next)).unwrap();
            }

            // drop reference to current before dropping result sender
            drop(current);
            drop(result_send);
        }
    }

}

impl<C> AutomataTrait for Automata<C>
where C: 'static + Cell + Send + Sync + Default + PartialEq + Eq,
      C::Params: 'static + Send + Sync
{
    fn next(&mut self) {
        let range_sender_iter = self.chunks.iter()
            .zip(self.item_senders.iter().cycle());
        let (result_send, result_recv) = channel();
        for (range, sender) in range_sender_iter {
            let item = WorkerItem {
                current: Arc::clone(&self.current),
                range: Range::clone(&range),
                result_send: Sender::clone(&result_send),
            };

            // SAFETY: receiver is not dropped until the sender is dropped
            sender.send(item).unwrap();
        }

        // drop the extra sender used to clone
        drop(result_send);

        // receive results from worker threads
        while let Ok((index, cell)) = result_recv.recv() {
            let next_cell = &mut self.next[index];
            *next_cell = cell;

            // if the cell is different from last generation, update the texture
            let curr_cell = &self.current[index];
            if self.texture.first_update || *next_cell != *curr_cell {
                let (x, y) = linear_to_grid(self.grid_size.rows, index);
                let color = next_cell.color(&self.cell_params);
                self.texture.set_pixel(x, y, color);
            }
        }

        // SAFETY: results are back from worker threads and additional
        // references were dropped
        let current = Arc::get_mut(&mut self.current).unwrap();
        std::mem::swap(current, &mut self.next);

        // update complete
        self.texture.first_update = false;
    }

    fn render(&self) {
        self.texture.update();
        self.texture.fill_screen();
    }
}

impl AutomataTexture {
    fn update(&self) {
        self.texture.update(&self.image);
    }

    fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.image.set_pixel(x as u32, y as u32, color);
    }

    fn fill_screen(&self) {
        draw_texture_ex(
            &self.texture,
            0.0,
            0.0,
            WHITE,
            self.params.clone()
        );
    }
}

fn linear_to_grid(rows: usize, index: usize) -> (usize, usize) {
    let col = index / rows;
    let row = index % rows;
    (col, row)
}

fn grid_to_linear(rows: usize, col: usize, row: usize) -> usize {
    col * rows + row
}
