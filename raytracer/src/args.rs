pub fn parse_args() -> (i32, i32, String) {
    match std::env::args().collect::<Vec<String>>().as_slice() {
        [_, a, b, c] => match (a.parse::<i32>(), b.parse::<i32>(), c.to_string()) {
            (Ok(a), Ok(b), c) => (a, b, c),
            _ => panic!("Incorrect command line args given"),
        },
        _ => panic!("Incorrect command line args given"),
    }
}
