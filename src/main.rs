#![allow(dead_code)]
#![allow(unused_variables)]

use std::{cell::RefCell, collections::btree_map::Iter, ops::Range};

struct State {
    grid: Vec<Vec<bool>>,
    fps: u32,
    running: bool,
    grid_size: (usize, usize),
}

impl State {
    fn new(grid_size: (usize, usize)) -> Self {
        State {
            grid: vec![vec![false; grid_size.1]; grid_size.0],
            fps: 1,
            running: false,
            grid_size,
        }
    }

    fn update(&mut self) {
        let xs = 0..self.grid_size.0;
        let ys = 0..self.grid_size.1;
        let mut cells_to_update: Vec<(usize, usize)> = Vec::new();
        let cells = cartesian(xs, ys);

        for cell in cells {
            let life_status = self.grid[cell.0][cell.1];
            let alive_neighbors = self.find_live_neighbor_count(cell);
            match life_status {
                true => match alive_neighbors {
                    n if n < 2 => cells_to_update.push(cell),
                    n if n == 2 || n == 3 => (),
                    n if n > 3 => cells_to_update.push(cell),
                    _ => unreachable!("this should never be reached"),
                },
                false => {
                    if alive_neighbors == 3 {
                        cells_to_update.push(cell);
                    }
                }
            }
        }

        cells_to_update
            .iter()
            .for_each(|cell| self.flip_cell(*cell));
    }

    fn flip_cell(&mut self, cell: (usize, usize)) {
        assert!(self.coord_is_legal(cell));
        let val = self.grid[cell.0][cell.1];
        self.grid[cell.0][cell.1] = !val;
    }

    fn find_live_neighbor_count(&self, cell: (usize, usize)) -> u8 {
        assert!(self.coord_is_legal(cell));
        let x = cell.0 as i32;
        let y = cell.1 as i32;
        let xs = -1..2;
        let ys = -1..2;
        //  let neighbor_list1 = cartesian(-1..2, -1..2)
        //neighbor_list is a list of all coordinates that are inside the board surounding the given cell.
        let neighbor_list: Vec<(i32, i32)> = cartesian(xs, ys)
            .map(|neighbor| (x + neighbor.0, y + neighbor.1))
            .filter(|(neighbor_x, neighbor_y)| {
                neighbor_x >= &0
                    && neighbor_y >= &0
                    && neighbor_x < &(self.grid_size.0 as i32)
                    && neighbor_y < &(self.grid_size.0 as i32)
            })
            .filter(|neighbor| neighbor != &(x, y))
            .collect();

        // counting the number of neighbors alive in the neighbor list.
        let num_alive = neighbor_list
            .iter()
            .map(|neighbor| self.grid[neighbor.0 as usize][neighbor.1 as usize])
            .filter(|life_status| *life_status)
            .count();
        num_alive as u8
    }

    fn coord_is_legal(&self, cell: (usize, usize)) -> bool {
        cell.0 < self.grid_size.0 && cell.1 < self.grid_size.1
    }

    fn validate(&self, assumption_list: &Vec<((usize, usize), bool)>) -> bool {
        for ((x, y), assumed_state) in assumption_list {
            if self.grid[*x][*y] != *assumed_state {
                return false;
            }
        }
        return true;
    }
}

fn cartesian<T: Clone + Copy>(xs: Range<T>, ys: Range<T>) -> impl Iterator<Item = (T, T)>
where
    Range<T>: Iterator<Item = T>,
{
    // takes in two ranges and returns all combinations of those ranges.
    ys.flat_map(move |y| xs.clone().map(move |x| (x, y)))
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod state_testing {
    use super::*;

    #[test]
    fn count_neighbors() {
        let mut state = State::new((3, 3));
        assert_eq!(0, state.find_live_neighbor_count((1, 1)));
        state.flip_cell((0, 0));
        assert_eq!(1, state.find_live_neighbor_count((1, 1)));
        assert_eq!(0, state.find_live_neighbor_count((0, 0)));
        state.flip_cell((2, 1));
        assert_eq!(1, state.find_live_neighbor_count((2, 2)));
    }


    #[test]
    fn test_stills(){
        let block: Vec<((usize, usize), bool)> = vec![
            ((0, 0), false),
            ((1, 0), false),
            ((2, 0), false),
            ((3, 0), false),
            ((0, 1), false),
            ((1, 1), true),
            ((2, 1), true),
            ((3, 1), false),
            ((0, 2), false),
            ((1, 2), true),
            ((2, 2), true),
            ((3, 2), false),
            ((0, 3), false),
            ((1, 3), false),
            ((2, 3), false),
            ((3, 3), false),
        ];

        let loaf: Vec<((usize, usize), bool)> = vec![
            ((0, 0), false),
            ((0, 1), false),
            ((0, 2), false),
            ((0, 3), false),
            ((0, 4), false),
            ((0, 5), false),
            ((1, 0), false),
            ((1, 1), false),
            ((1, 2), false),
            ((1, 3), true),
            ((1, 4), false),
            ((1, 5), false),
            ((2, 0), false),
            ((2, 1), false),
            ((2, 2), true),
            ((2, 3), false),
            ((2, 4), true),
            ((2, 5), false),
            ((3, 0), false),
            ((3, 1), true),
            ((3, 2), false),
            ((3, 3), false),
            ((3, 4), true),
            ((3, 5), false),
            ((4, 0), false),
            ((4, 1), false),
            ((4, 2), true),
            ((4, 3), true),
            ((4, 4), false),
            ((4, 5), false),
            ((5, 0), false),
            ((5, 1), false),
            ((5, 2), false),
            ((5, 3), false),
            ((5, 4), false),
            ((5, 5), false),
        ];

        [block, loaf]
            .iter()
            .for_each(|cell_list| {
                let mut state = State::new((30, 30));
                println!("{:?}", state.grid);
                cell_list
                    .iter()
                    .filter(|(_, assumption)| *assumption)
                    .for_each(|(cell, _)| state.flip_cell(*cell));
        
                for _ in 0..10 {
                    state.update();
                    assert!(state.validate(&cell_list));
                }
            })

    }


    #[test]
    fn still_block() {
        let mut state = State::new((4, 4));
        let cells: Vec<((usize, usize), bool)> = vec![
            ((0, 0), false),
            ((1, 0), false),
            ((2, 0), false),
            ((3, 0), false),
            ((0, 1), false),
            ((1, 1), true),
            ((2, 1), true),
            ((3, 1), false),
            ((0, 2), false),
            ((1, 2), true),
            ((2, 2), true),
            ((3, 2), false),
            ((0, 3), false),
            ((1, 3), false),
            ((2, 3), false),
            ((3, 3), false),
        ];
        cells
            .iter()
            .filter(|(_, assumption)| *assumption)
            .for_each(|(cell, _)| state.flip_cell(*cell));

        for _ in 0..10 {
            state.update();
            assert!(state.validate(&cells));
        }
    }

    #[test]
    fn still_loaf() {
        let mut state = State::new((6, 6));
        let cells: Vec<((usize, usize), bool)> = vec![
            ((0, 0), false),
            ((0, 1), false),
            ((0, 2), false),
            ((0, 3), false),
            ((0, 4), false),
            ((0, 5), false),
            ((1, 0), false),
            ((1, 1), false),
            ((1, 2), false),
            ((1, 3), true),
            ((1, 4), false),
            ((1, 5), false),
            ((2, 0), false),
            ((2, 1), false),
            ((2, 2), true),
            ((2, 3), false),
            ((2, 4), true),
            ((2, 5), false),
            ((3, 0), false),
            ((3, 1), true),
            ((3, 2), false),
            ((3, 3), false),
            ((3, 4), true),
            ((3, 5), false),
            ((4, 0), false),
            ((4, 1), false),
            ((4, 2), true),
            ((4, 3), true),
            ((4, 4), false),
            ((4, 5), false),
            ((5, 0), false),
            ((5, 1), false),
            ((5, 2), false),
            ((5, 3), false),
            ((5, 4), false),
            ((5, 5), false),
        ];

        cells
            .iter()
            .filter(|(_, assumption)| *assumption)
            .for_each(|(cell, _)| state.flip_cell(*cell));

        for _ in 0..10 {
            state.update();
            assert!(state.validate(&cells));
        }
    }
}
