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
    let mut col = 3;
    let mut trees = 0;

    for row in input.iter().skip(1) {
        if row[col] {
            trees += 1;
        }
        col = (col + 3) % row.len();
    }

    trees
}
