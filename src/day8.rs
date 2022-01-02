#[derive(Debug, Default)]
struct Pattern([bool; 7]);

#[derive(Debug, Default)]
struct Line {
    patterns: [Pattern; 10],
    output: [Pattern; 4],
}

impl Pattern {
    fn parse(s: &str) -> Pattern {
        let mut pattern = [false; 7];
        for (i, c) in ('a'..='g').enumerate() {
            if s.contains(c) {
                pattern[i] = true;
            }
        }
        Pattern(pattern)
    }

    fn is_1478(&self) -> bool {
        let count: usize = self
            .0
            .iter()
            .map(|&p| if p { 1usize } else { 0usize })
            .sum();
        count == 2 || count == 3 || count == 4 || count == 7
    }
}

impl Line {
    fn parse(s: &str) -> Line {
        let mut it = s.split(" | ");
        let p = it.next().unwrap();
        let o = it.next().unwrap();

        let patterns = p
            .split(" ")
            .map(|w| Pattern::parse(w))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let output = o
            .split(" ")
            .map(|w| Pattern::parse(w))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        // dbg!(&p, &patterns);
        Line { patterns, output }
    }

    fn count_1478_outputs(&self) -> usize {
        self.output
            .iter()
            .map(|p| if p.is_1478() { 1usize } else { 0usize })
            .sum()
    }
}

// number of segments on:
// 0: 6
// 1: 2 <--
// 2: 5
// 3: 5
// 4: 4 <--
// 5: 5
// 6: 6
// 7: 3 <--
// 8: 7 <--
// 9: 6

fn main() {
    let path = std::env::args().nth(1).expect("pls provide input file");
    let contents = std::fs::read_to_string(path).expect("read failed");

    let lines: Vec<_> = contents.lines().map(|x| Line::parse(x)).collect();
    let output_1478_count: usize = lines.iter().map(|l| l.count_1478_outputs()).sum();
    dbg!(&output_1478_count);
}
