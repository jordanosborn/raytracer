pub fn parse_args() -> (i32, i32, i32, String) {
    match std::env::args().collect::<Vec<String>>().as_slice() {
        [_, a, b, c, d] => match (
            a.parse::<i32>(),
            b.parse::<i32>(),
            c.parse::<i32>(),
            d.to_string(),
        ) {
            (Ok(a), Ok(b), Ok(c), d) => (a, b, c, d),
            _ => panic!("Incorrect command line args given"),
        },
        _ => panic!("Incorrect command line args given"),
    }
}
