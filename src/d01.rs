pub fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = i32> + 'a {
    input
        .split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse().ok())
}
