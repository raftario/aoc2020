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
        for (field, value) in &self.0 {
            if !field.is_valid(value) {
                return false;
            }
        }

        self.0.contains_key(&Field::BirthYear)
            && self.0.contains_key(&Field::IssueYear)
            && self.0.contains_key(&Field::ExpirationYear)
            && self.0.contains_key(&Field::Height)
            && self.0.contains_key(&Field::HairColour)
            && self.0.contains_key(&Field::EyeColour)
            && self.0.contains_key(&Field::PassportId)
    }
}

impl Field {
    fn is_valid(self, value: &str) -> bool {
        match self {
            Field::BirthYear => {
                value.len() == 4
                    && matches!(value.parse::<u32>(), Ok(yr) if 1920 <= yr && yr <= 2002)
            }
            Field::IssueYear => {
                value.len() == 4
                    && matches!(value.parse::<u32>(), Ok(yr) if 2010 <= yr && yr <= 2020)
            }
            Field::ExpirationYear => {
                value.len() == 4
                    && matches!(value.parse::<u32>(), Ok(yr) if 2020 <= yr && yr <= 2030)
            }
            Field::Height => match (
                value[..(value.len() - 2)].parse::<u32>(),
                &value[(value.len() - 2)..],
            ) {
                (Ok(h), "cm") if 150 <= h && h <= 193 => true,
                (Ok(h), "in") if 59 <= h && h <= 76 => true,
                _ => false,
            },
            Field::HairColour => {
                value.len() == 7
                    && value.as_bytes()[0] == b'#'
                    && value[1..].chars().all(|c| c.is_ascii_hexdigit())
            }
            Field::EyeColour => {
                matches!(value, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
            }
            Field::PassportId => value.len() == 9 && value.parse::<u32>().is_ok(),
            Field::CountryId => true,
        }
    }
}
