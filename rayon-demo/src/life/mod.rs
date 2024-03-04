use crate::cpu_time::{self, CpuMesure};
use rand::distributions::Standard;
use rand::{thread_rng, Rng};
use std::iter::repeat;
use std::mem::take;
use std::sync::Arc;

#[cfg(test)]
mod bench;

#[derive(serde::Deserialize)]
pub struct Args {
    cmd_bench: bool,
    cmd_play: bool,
    flag_size: usize,
    flag_gens: usize,
    flag_fps: usize,
    flag_skip_bridge: bool,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Board {
    board: Vec<bool>,
    survive: Arc<Vec<usize>>,
    born: Arc<Vec<usize>>,
    rows: usize,
    cols: usize,
}

impl Board {
    pub fn new(rows: usize, cols: usize) -> Board {
        let born = vec![3];
        let survive = vec![2, 3];

        Board::new_with_custom_rules(rows, cols, born, survive)
    }

    fn new_with_custom_rules(
        rows: usize,
        cols: usize,
        born: Vec<usize>,
        survive: Vec<usize>,
    ) -> Board {
        let new_board = repeat(false).take(rows * cols).collect();

        Board {
            board: new_board,
            survive: Arc::new(survive),
            born: Arc::new(born),
            rows,
            cols,
        }
    }

    fn len(&self) -> usize {
        self.rows * self.cols
    }

    fn next_board(&self, new_board: Vec<bool>) -> Board {
        assert!(new_board.len() == self.len());

        Board {
            board: new_board,
            survive: self.survive.clone(),
            born: self.born.clone(),
            rows: self.rows,
            cols: self.cols,
        }
    }

    pub fn random(&self) -> Board {
        let new_brd = thread_rng()
            .sample_iter(&Standard)
            .take(self.len())
            .collect();

        self.next_board(new_brd)
    }

    pub fn next_generation(&self) -> Board {
        let new_brd = (0..self.len())
            .map(|cell| self.successor_cell(cell))
            .collect();
        self.next_board(new_brd)
    }

    pub fn parallel_next_generation(&self) -> Board {
        let new_brd = (0..self.len())
            .into_par_iter()
            .map(|cell| self.successor_cell(cell))
            .collect();
    }

    fn successor_cell(&self, cell: usize) -> bool {
        self.successor(cell % self.cols, cell / self.cols)
    }

    fn successor(&self, x: usize, y: usize) -> bool {
        let neighbors = self.living_neighbors(x, y);

        if self.cell_live(x, y) {
            self.survive.contains(&neighbors)
        } else {
            self.born.contains(&neighbors)
        }
    }

    fn living_neighbors(&self, x: usize, y: usize) -> usize {
        let x_1 = x.wrapping_sub(1);
        let y_1 = y.wrapping_sub(1);
        let neighbors = [
            self.cell_live(x_1, y_1),
            self.cell_live(x, y_1),
            self.cell_live(x + 1, y_1),
            self.cell_live(x_1, y),
            self.cell_live(x + 1, y),
            self.cell_live(x_1, y + 1),
            self.cell_live(x, y + 1),
            self.cell_live(x + 1, y + 1),
        ];
        neighbors.iter().filter(|&x| *x).count()
    }

    fn cell_live(&self, x: usize, y: usize) -> bool {
        !(x >= self.cols || y >= self.rows) && self.board[y * self.cols + x]
    }
}
