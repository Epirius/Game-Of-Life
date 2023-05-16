use crate::game::State;


mod game;

fn main() {
    println!("Hello, world!");

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

    let mut state = State::new((10, 10));
    start
        .iter()
        .filter(|(_, assumption)| *assumption)
        .for_each(|(cell, _)| state.flip_cell(*cell));

    for _ in 0..20 {
        state.print();
        state.update();
    }
}
