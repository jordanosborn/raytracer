pub fn parse_args() -> (i32, i32) {
    match std::env::args().collect::<Vec<String>>().as_slice() {
        [_, a, b] => match (a.parse::<i32>(), b.parse::<i32>()) {
            (Ok(a), Ok(b)) => (a, b),
            _ => panic!("Incorrect command line args given"),
        },
        _ => panic!("Incorrect command line args given"),
    }
}
