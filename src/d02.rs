pub struct Policy {
    pub n1: usize,
    pub n2: usize,
    pub letter: char,
}

impl From<&str> for Policy {
    fn from(s: &str) -> Self {
        let mut numbers_and_letter = s.split(' ');

        let mut numbers = numbers_and_letter.next().unwrap().split('-');
        let n1 = numbers.next().unwrap().parse().unwrap();
        let n2 = numbers.next().unwrap().parse().unwrap();

        let letter = numbers_and_letter.next().unwrap().parse().unwrap();

        Self { n1, n2, letter }
    }
}

pub fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = (Policy, &'a str)> + 'a {
    input.split('\n').filter(|s| !s.is_empty()).map(|s| {
        let mut policy_and_password = s.split(": ");
        let policy = policy_and_password.next().unwrap().into();
        let password = policy_and_password.next().unwrap();
        (policy, password)
    })
}
