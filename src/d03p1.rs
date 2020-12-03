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
    let len = input.first().unwrap().len();

    let rows = input.iter().skip(1);
    let cols = (0..len).cycle().step_by(3).skip(1);

    rows.zip(cols).filter(|(row, col)| row[*col]).count()
}
