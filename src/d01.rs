pub fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = i32> + 'a {
    input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
}
