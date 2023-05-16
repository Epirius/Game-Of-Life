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
        handle_key_pressed(&mut game);
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

fn handle_key_pressed(game: &mut State){
    if is_key_released(KeyCode::Space) {
        game.running = !game.running;
    }
    if is_key_released(KeyCode::Right) && !game.running {
         game.running = true;
         game.update();
         game.running = false;
    }
}

fn init(grid_size: (usize, usize)) -> State {
    let start: Vec<((usize, usize), bool)> = vec![
        ((10, 1), true),
        ((11, 1), true),
        ((10, 2), true),
        ((11, 2), true),
        ((10, 11), true),
        ((11, 11), true),
        ((12, 11), true),
        ((9, 12), true),
        ((13, 12), true),
        ((14, 13), true),
        ((14, 14), true),
        ((8, 13), true),
        ((8, 14), true),
        ((11, 15), true),
        ((9, 16), true),
        ((13, 16), true),
        ((10, 17), true),
        ((11, 17), true),
        ((12, 17), true),
        ((11, 18), true),
        ((10, 21), true),
        ((9, 21), true),
        ((8, 21), true),
        ((10, 22), true),
        ((9, 22), true),
        ((8, 22), true),
        ((7, 23), true),
        ((11, 23), true),
        ((7, 25), true),
        ((11, 25), true),
        ((6, 25), true),
        ((12, 25), true),


        ((8, 35), true),
        ((9, 35), true),
        ((8, 36), true),
        ((9, 36), true),
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
