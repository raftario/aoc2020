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
        let c1 = password.chars().nth(self.n1 - 1);
        let c2 = password.chars().nth(self.n2 - 1);

        matches!(
            (c1 == Some(self.letter), c2 == Some(self.letter)),
            (true, false) | (false, true)
        )
    }
}
