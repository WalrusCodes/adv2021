#[derive(Debug, Default)]
struct Pattern([bool; 7]);

#[derive(Debug, Default)]
struct Line {
    patterns: [Pattern; 10],
    value: [Pattern; 4],
}

impl Line {
    fn parse(s: &str) -> Line {
        Line::default()
    }
}

fn main() {
    let path = std::env::args().nth(1).expect("pls provide input file");
    let contents = std::fs::read_to_string(path).expect("read failed");

    contents.lines().map(|x| Line::parse(x));
}
