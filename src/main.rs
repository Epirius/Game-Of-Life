
const GRID_SIZE: (f32, f32) = (100.0, 100.0);
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
    
    fn flip_cell(&mut self, cell: (usize, usize)) {
        assert!(self.coord_is_legal(cell));
        let val = self.grid[cell.0][cell.1];
        self.grid[cell.0][cell.1] = !val;
    }
    
    fn find_live_neighbor_count(&self, cell: (usize, usize)) -> u8 {
        assert!(self.coord_is_legal(cell));
        let x = cell.0 as i32;
        let y = cell.1 as i32;
        let xs = -1..=1;
        let ys = -1..=1;
        //neighbor_list is a list of all coordinates that are inside the board surounding the given cell. 
        let neighbor_list: Vec<(i32, i32)> = ys
            .flat_map(|y| xs.clone().map(move |x| (x, y)))
            .map(|neighbor| (x + neighbor.0, y + neighbor.1))
            .filter(|(neighbor_x, neighbor_y)| 
                neighbor_x >= &0 
                && neighbor_y >= &0
                && neighbor_x < &(self.grid_size.0 as i32)
                && neighbor_y < &(self.grid_size.0 as i32)
            )
            .filter(|neighbor| neighbor != & (x, y))
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
}


fn main() {
    println!("Hello, world!");
}     


#[cfg(test)]
mod state_testing {
    use super::*;

    #[test]
    fn count_neighbors() {
        let mut state = State::new((3,3));
        assert_eq!(0, state.find_live_neighbor_count((1,1)));
        state.flip_cell((0,0));
        assert_eq!(1, state.find_live_neighbor_count((1,1)));
        assert_eq!(0, state.find_live_neighbor_count((0,0)));
        state.flip_cell((2,1));
        assert_eq!(1, state.find_live_neighbor_count((2,2)));
    }
}