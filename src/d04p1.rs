mod d04;

use std::{env, fs};

use d04::{Field, Passport};

fn main() {
    let input = env::args().nth(1).unwrap();
    let input = fs::read_to_string(input).unwrap();
    let input = d04::parse_input(&input);

    let res = run(input);
    println!("{}", res);
}

fn run<'a>(input: impl Iterator<Item = Passport<'a>> + 'a) -> usize {
    input.filter(Passport::is_valid).count()
}

impl Passport<'_> {
    fn is_valid(&self) -> bool {
        self.0.contains_key(&Field::BirthYear)
            && self.0.contains_key(&Field::IssueYear)
            && self.0.contains_key(&Field::ExpirationYear)
            && self.0.contains_key(&Field::Height)
            && self.0.contains_key(&Field::HairColour)
            && self.0.contains_key(&Field::EyeColour)
            && self.0.contains_key(&Field::PassportId)
    }
}
