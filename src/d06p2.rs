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
                .fold(
                    Option::<HashSet<char>>::None,
                    |every, current| match every {
                        Some(every) => Some(every.intersection(&current).copied().collect()),
                        None => Some(current),
                    },
                )
                .unwrap()
                .len()
        })
        .sum()
}
