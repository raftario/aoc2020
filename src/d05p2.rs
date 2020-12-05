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
    let mut seats = [false; 0b1111111111];
    for n in input {
        seats[n] = true;
    }
    for (n, taken) in seats.iter().enumerate() {
        if let (Some(true), false, Some(true)) = (seats.get(n - 1), taken, seats.get(n + 1)) {
            return n;
        }
    }

    panic!("no result")
}
