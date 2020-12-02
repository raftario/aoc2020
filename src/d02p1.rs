mod d02;

use std::{env, fs};

use d02::Policy;

fn main() {
    let input = env::args().nth(1).unwrap();
    let input = fs::read_to_string(input).unwrap();
    let input = d02::parse_input(&input);

    let res = run(input);
    println!("{}", res);
}

fn run<'a>(input: impl Iterator<Item = (Policy, &'a str)> + 'a) -> usize {
    input
        .filter(|(policy, password)| policy.is_valid(password))
        .count()
}

impl Policy {
    fn is_valid(&self, password: &str) -> bool {
        let occurrences = password.chars().filter(|c| *c == self.letter).count();
        self.n1 <= occurrences && occurrences <= self.n2
    }
}
