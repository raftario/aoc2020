mod d05;

use std::{env, fs};

fn main() {
    let input = env::args().nth(1).unwrap();
    let input = fs::read_to_string(input).unwrap();
    let input = d05::parse_input(&input);

    let res = run(input);
    println!("{}", res);
}

fn run<'a>(input: impl Iterator<Item = usize> + 'a) -> usize {
    input.fold(0, |max, n| max.max(n))
}
