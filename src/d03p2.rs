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
    let mut total = 1;

    for (right, down) in slopes.iter().copied() {
        let mut col = right;
        let mut trees = 0;

        for row in input.iter().step_by(down).skip(1) {
            if row[col] {
                trees += 1;
            }
            col = (col + right) % row.len();
        }

        total *= trees;
    }

    total
}
