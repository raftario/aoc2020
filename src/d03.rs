pub fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.chars()
                .map(|c| match c {
                    '.' => false,
                    '#' => true,
                    c => panic!("invalid input: expected '.' or '#', got '{}'", c),
                })
                .collect()
        })
        .collect()
}
