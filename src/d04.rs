use std::collections::HashMap;

pub struct Passport<'a>(pub HashMap<Field, &'a str>);

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Field {
    BirthYear,
    IssueYear,
    ExpirationYear,
    Height,
    HairColour,
    EyeColour,
    PassportId,
    CountryId,
}

impl<'a> From<&'a str> for Passport<'a> {
    fn from(s: &'a str) -> Self {
        Self(
            s.split(&['\n', ' '][..])
                .filter(|s| !s.is_empty())
                .map(|s| {
                    let mut field_and_value = s.split(':');
                    let field = field_and_value.next().unwrap().into();
                    let value = field_and_value.next().unwrap();
                    (field, value)
                })
                .collect(),
        )
    }
}

impl From<&str> for Field {
    fn from(s: &str) -> Self {
        match s {
            "byr" => Self::BirthYear,
            "iyr" => Self::IssueYear,
            "eyr" => Self::ExpirationYear,
            "hgt" => Self::Height,
            "hcl" => Self::HairColour,
            "ecl" => Self::EyeColour,
            "pid" => Self::PassportId,
            "cid" => Self::CountryId,
            s => panic!("invalid field: {}", s),
        }
    }
}

pub fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = Passport<'a>> + 'a {
    input
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(Into::into)
}
