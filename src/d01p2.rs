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
    for (i1, n1) in input.iter().copied().enumerate() {
        for (i2, n2) in input.iter().skip(i1 + 1).copied().enumerate() {
            for n3 in input.iter().skip(i2 + 1).copied() {
                if n1 + n2 + n3 == 2020 {
                    return n1 * n2 * n3;
                }
            }
        }
    }

    panic!("no result")
}
