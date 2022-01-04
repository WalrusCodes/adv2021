use std::{collections::HashMap, str::Lines};

#[derive(Default, Debug)]
struct State {
    pairs: HashMap<(char, char), usize>,
    last: char,
}

#[derive(Default, Debug)]
struct Mapping(HashMap<(char, char), char>);

impl State {
    fn parse(line: &str) -> State {
        let mut pairs: HashMap<(char, char), usize> = HashMap::new();
        for pair in line.chars().zip(line.chars().skip(1)) {
            *pairs.entry(pair).or_insert(0) += 1;
        }
        State {
            pairs,
            last: line.chars().last().unwrap(),
        }
    }

    fn step(&self, mapping: &Mapping) -> State {
        let mut pairs: HashMap<(char, char), usize> = HashMap::new();
        for (pair, count) in self.pairs.iter() {
            let to = mapping.0.get(pair).unwrap();
            *pairs.entry((pair.0, *to)).or_insert(0) += count;
            *pairs.entry((*to, pair.1)).or_insert(0) += count;
        }
        State {
            pairs,
            last: self.last,
        }
    }

    fn count_max_minus_min(&self) -> usize {
        let mut counts: HashMap<char, usize> = HashMap::new();
        for (&(a, _), count) in self.pairs.iter() {
            *counts.entry(a).or_insert(0) += count;
        }
        // re-add last item
        *counts.entry(self.last).or_insert(0) += 1;
        let max = counts.values().max().unwrap();
        let min = counts.values().min().unwrap();
        max - min
    }
}

impl Mapping {
    fn parse_rules(lines: Lines) -> Mapping {
        Mapping(HashMap::from_iter(
            lines.map(|line| Mapping::parse_rule(line)),
        ))
    }

    fn parse_rule(line: &str) -> ((char, char), char) {
        let mut chars = line.chars();
        dbg!(&chars);
        (
            (chars.nth(0).unwrap(), chars.nth(0).unwrap()),
            chars.nth(4).unwrap(),
        )
    }
}

fn parse(contents: &str) -> (State, Mapping) {
    let mut it = contents.lines();
    let state = State::parse(it.next().unwrap());
    it.next(); // skip blank line
    let mapping = Mapping::parse_rules(it);
    (state, mapping)
}

fn main() {
    let path = std::env::args().nth(1).expect("pls provide input file");
    let contents = std::fs::read_to_string(path).expect("read failed");

    let (mut state, mapping) = parse(&contents);
    for i in 0..40 {
        state = state.step(&mapping);
        if i == 9 || i == 39 {
            dbg!(state.count_max_minus_min());
        }
    }
}
