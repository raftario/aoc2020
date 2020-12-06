mod d06;

use std::{collections::HashSet, env, fs};

fn main() {
    let input = env::args().nth(1).unwrap();
    let input = fs::read_to_string(input).unwrap();
    let input = d06::parse_input(&input);

    let res = run(input);
    println!("{}", res);
}

fn run<'a>(input: impl Iterator<Item = impl Iterator<Item = HashSet<char>> + 'a> + 'a) -> usize {
    input
        .map(|group| {
            group
                .fold(HashSet::new(), |mut any, current| {
                    any.extend(current);
                    any
                })
                .len()
        })
        .sum()
}
