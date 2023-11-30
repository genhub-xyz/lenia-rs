use crate::cell::Cell;
use rayon::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl From<(usize, usize)> for Point {
    fn from(item: (usize, usize)) -> Self {
        Self {
            x: item.0,
            y: item.1,
        }
    }
}

pub struct Grid {
    width: usize,
    height: usize,
    pub cells: Vec<Cell>,
}

impl Grid {
    // Width and height of the Grid
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![Cell::new(false); width * height],
        }
    }

    pub fn set_state(&mut self, cells_coords: &[Point]) {
        self.cells = vec![Cell::new(false); self.width * self.height];
        for &pos in cells_coords.iter() {
            let idx = self.coords_to_index(pos);
            self.cells[idx].set_state(true);
        }
    }

    fn cell_next_state(&self, cell_idx: usize) -> bool {
        let cell = self.cells[cell_idx].clone();
        let cell_pos = self.index_to_coords(cell_idx);
        // Check boundaries and add neighgours
        let mut num_neighbour_alive = 0;
        for &x_off in [-1, 0, 1isize].iter() {
            for &y_off in [-1, 0, 1isize].iter() {
                if x_off == 0 && y_off == 0 {
                    continue;
                }

                let neighbour_coords = (cell_pos.x as isize + x_off, cell_pos.y as isize + y_off);
                let is_offscreen_left = neighbour_coords.0 < 0;
                let is_offscreen_right = neighbour_coords.0 > (self.width - 1) as isize;
                let is_offscreen_top = neighbour_coords.1 < 0;
                let is_offsreen_bottom = neighbour_coords.1 > (self.height - 1) as isize;
                let is_offscreen = is_offscreen_left
                    || is_offscreen_right
                    || is_offscreen_top
                    || is_offsreen_bottom;

                if is_offscreen {
                    continue;
                }

                let neighbour_pos = Point {
                    x: neighbour_coords.0.unsigned_abs(),
                    y: neighbour_coords.1.unsigned_abs(),
                };
                let idx = self.coords_to_index(neighbour_pos);
                if self.cells[idx].is_alive() {
                    num_neighbour_alive += 1;
                }
            }
        }

        // Rules (from wikipedia)
        if cell.is_alive() && (num_neighbour_alive == 2 || num_neighbour_alive == 3) {
            return true; // alive
        }
        if !cell.is_alive() && num_neighbour_alive == 3 {
            return true;
        }

        false
    }

    pub fn update(&mut self) {
        // Vector of next states. It will match by index
        // Get next states
        // Iterative lags, parallel stronk
        let next_states = (0..self.cells.len())
            .into_par_iter()
            .map(|idx| {
                // next state
                self.cell_next_state(idx)
            })
            .collect::<Vec<bool>>();

        // Update states
        self.cells = (0..self.cells.len())
            .into_par_iter()
            .map(|idx| Cell::new(next_states[idx]))
            .collect::<Vec<Cell>>();
    }

    /// Converts a pair of cell coords to index in the cells vector
    pub fn coords_to_index(&self, pos: Point) -> usize {
        pos.y * self.height + pos.x
    }

    /// Converts a index in the cells vecotr into pair of cell coords
    pub fn index_to_coords(&self, index: usize) -> Point {
        Point {
            x: index % self.width,
            y: index / self.height,
        }
    }
}
