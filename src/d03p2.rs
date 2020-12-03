mod d03;

use std::{env, fs};

fn main() {
    let input = env::args().nth(1).unwrap();
    let input = fs::read_to_string(input).unwrap();
    let input = d03::parse_input(&input);

    let res = run(input);
    println!("{}", res);
}

fn run(input: Vec<Vec<bool>>) -> usize {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let len = input.first().unwrap().len();
    let mut total = 1;

    for (right, down) in slopes.iter().copied() {
        let rows = input.iter().step_by(down).skip(1);
        let cols = (0..len).cycle().step_by(right).skip(1);

        total *= rows.zip(cols).filter(|(row, col)| row[*col]).count();
    }

    total
}
