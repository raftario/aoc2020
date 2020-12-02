mod d01;

use std::{env, fs};

fn main() {
    let input = env::args().nth(1).unwrap();
    let input = fs::read_to_string(input).unwrap();
    let input: Vec<i32> = d01::parse_input(&input).collect();

    let res = run(&input);
    println!("{}", res);
}

fn run(input: &[i32]) -> i32 {
    for (i, n1) in input.iter().copied().enumerate() {
        for n2 in input.iter().skip(i + 1).copied() {
            if n1 + n2 == 2020 {
                return n1 * n2;
            }
        }
    }

    panic!("no result")
}
