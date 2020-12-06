use std::collections::HashSet;

pub fn parse_input<'a>(
    input: &'a str,
) -> impl Iterator<Item = impl Iterator<Item = HashSet<char>> + 'a> + 'a {
    input.split("\n\n").filter(|s| !s.is_empty()).map(|s| {
        s.split('\n')
            .filter(|s| !s.is_empty())
            .map(|s| s.chars().collect())
    })
}
