pub fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = usize> + 'a {
    input.split('\n').filter(|s| !s.is_empty()).map(|s| {
        s.as_bytes()
            .iter()
            .rev()
            .enumerate()
            .fold(0, |n, (i, c)| match (c, i) {
                (b'R', 0..=2) | (b'B', 3..=9) => n | (1 << i),
                (b'L', 0..=2) | (b'F', 3..=9) => n,
                _ => panic!("invalid character '{}' at position {}", char::from(*c), i),
            })
    })
}
