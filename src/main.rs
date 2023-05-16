use crate::game::State;
mod game;

use macroquad::prelude::*;

const SQUARE_SIZE: (f32, f32) = (20.0, 20.0);
const GRID_SIZE: (usize, usize) = (100, 100);

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut game = init(GRID_SIZE);
    loop {
        clear_background(Color {
            r: 0.07,
            g: 0.07,
            b: 0.07,
            a: 1.0,
        });
        draw_board(&game.grid, SQUARE_SIZE);
        game.update();
        if (is_mouse_button_released(MouseButton::Left)){
            handle_mouse_click(&mut game, SQUARE_SIZE);
        }
        if (is_key_released(KeyCode::Space)){
            game.running = !game.running;
        }
        next_frame().await
    }
}

fn draw_board(grid: &Vec<Vec<bool>>, square_size: (f32, f32)) {
    for (y, line) in grid.iter().enumerate() {
        let y = y as f32;
        for (x, square) in line.iter().enumerate() {
            let x = x as f32;
            let color: Color = match square {
                true => GRAY,
                false => WHITE,
            };
            draw_rectangle(
                square_size.0 * x,
                square_size.1 * y,
                square_size.0 - 1.0,
                square_size.1 - 1.0,
                color,
            )
        }
    }
}

fn handle_mouse_click(game: &mut State, square_size: (f32, f32)){
    let (x, y) = mouse_position();
    let x = (x / square_size.0).floor() as usize;
    let y = (y / square_size.1).floor() as usize;

    if !game.running {
        game.flip_cell((y, x))
    }
}

fn init(grid_size: (usize, usize)) -> State {
    let start: Vec<((usize, usize), bool)> = vec![
        ((0, 0), false),
        ((1, 0), false),
        ((2, 0), false),
        ((3, 0), false),
        ((4, 0), false),
        ((0, 1), false),
        ((1, 1), true),
        ((2, 1), false),
        ((3, 1), false),
        ((4, 1), false),
        ((0, 2), false),
        ((1, 2), false),
        ((2, 2), true),
        ((3, 2), true),
        ((4, 2), false),
        ((0, 3), false),
        ((1, 3), true),
        ((2, 3), true),
        ((3, 3), false),
        ((4, 3), false),
        ((0, 4), false),
        ((1, 4), false),
        ((2, 4), false),
        ((3, 4), false),
        ((4, 4), false),
    ];

    let mut state = State::new(grid_size);
    start
        .iter()
        .filter(|(_, assumption)| *assumption)
        .for_each(|(cell, _)| state.flip_cell(*cell));
    /*
    for _ in 0..20 {
        state.print();
        state.update();
    }
     */
    state
}
