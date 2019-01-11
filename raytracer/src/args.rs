pub fn parse_args() -> (u32, u32, u32, String) {
    match std::env::args().collect::<Vec<String>>().as_slice() {
        [_, a, b, c, d] => match (
            a.parse::<u32>(),
            b.parse::<u32>(),
            c.parse::<u32>(),
            d.to_string(),
        ) {
            (Ok(a), Ok(b), Ok(c), d) => (a, b, c, d),
            _ => panic!("Incorrect command line args given"),
        },
        _ => panic!("Incorrect command line args given"),
    }
}
