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
    let mut total = 0;

    for (right, down) in slopes.iter().copied() {
        let mut col = 0;
        let mut trees = 0;

        for row in input.iter().step_by(down) {
            if row[col] {
                trees += 1;
            }
            col = (col + right) % row.len();
        }

        if total > 0 {
            total *= trees;
        } else {
            total = trees;
        }
    }

    total
}
