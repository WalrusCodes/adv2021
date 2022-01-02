mod day8_patterns;

use day8_patterns::{create_possible_permutations, Digit, Digits};

#[derive(Debug, Default)]
struct Line {
    digits: Digits,
    output: [Digit; 4],
}

impl Digit {
    fn parse(s: &str) -> Digit {
        let mut pattern = 0u8;
        for (i, c) in ('a'..='g').enumerate() {
            if s.contains(c) {
                pattern |= 1 << i;
            }
        }
        Digit(pattern)
    }

    fn is_1478(&self) -> bool {
        let count: u8 = (0..7).map(|i| (self.0 >> i) & 1).sum();

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

        count == 2 || count == 3 || count == 4 || count == 7
    }
}

impl Line {
    fn parse(s: &str) -> Line {
        let mut it = s.split(" | ");
        let p = it.next().unwrap();
        let o = it.next().unwrap();

        let mut digits = Digits(
            p.split(" ")
                .map(|w| Digit::parse(w))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        );
        digits.0.sort();
        let output = o
            .split(" ")
            .map(|w| Digit::parse(w))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Line { digits, output }
    }

    fn count_1478_outputs(&self) -> usize {
        self.output
            .iter()
            .map(|p| if p.is_1478() { 1usize } else { 0usize })
            .sum()
    }
}

fn main() {
    let path = std::env::args().nth(1).expect("pls provide input file");
    let contents = std::fs::read_to_string(path).expect("read failed");

    let lines: Vec<_> = contents.lines().map(|x| Line::parse(x)).collect();
    let output_1478_count: usize = lines.iter().map(|l| l.count_1478_outputs()).sum();
    dbg!(&output_1478_count);

    let perms = create_possible_permutations();
    let mut sum = 0u32;
    for line in lines.iter() {
        let value = perms.decode(&line.digits, &line.output);
        dbg!(&value);
        sum += value;
    }
    dbg!(sum);
}
