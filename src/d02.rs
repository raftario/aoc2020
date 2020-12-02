use std::{convert::Infallible, str::FromStr};

pub struct Policy {
    pub n1: usize,
    pub n2: usize,
    pub letter: char,
}

impl FromStr for Policy {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers_and_letter = s.split(' ');

        let mut numbers = numbers_and_letter.next().unwrap().split('-');
        let n1 = numbers.next().unwrap().parse().unwrap();
        let n2 = numbers.next().unwrap().parse().unwrap();

        let letter = numbers_and_letter.next().unwrap().parse().unwrap();

        Ok(Self { n1, n2, letter })
    }
}

pub fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = (Policy, &'a str)> + 'a {
    input.split('\n').filter(|s| !s.is_empty()).map(|s| {
        let mut policy_and_password = s.split(": ");
        let policy = policy_and_password.next().unwrap().parse().unwrap();
        let password = policy_and_password.next().unwrap();
        (policy, password)
    })
}
