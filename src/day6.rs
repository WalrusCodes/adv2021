#[derive(Debug)]
struct State {
    counts: [usize; 9],
    phase: usize,
}

impl State {
    fn parse(line: &str) -> State {
        let mut counts = [0usize; 9];
        for fish_str in line.split(",") {
            let idx = fish_str.parse::<usize>().unwrap();
            counts[idx] += 1;
        }

        State { counts, phase: 0 }
    }
    fn simulate_day(&mut self) {
        self.counts[(self.phase + 7) % 9] += self.counts[self.phase];
        self.phase = (self.phase + 1) % 9;
    }
    fn total(&self) -> usize {
        self.counts.iter().sum()
    }
}

fn main() {
    let path = std::env::args().nth(1).expect("pls provide input file");
    let contents = std::fs::read_to_string(path).expect("read failed");
    let mut state = State::parse(contents.trim());
    dbg!(&state);
    for _ in 0..80 {  // part 1
        state.simulate_day();
    }
    dbg!(&state, state.total());
    assert_eq!(state.total(), 385391);
    for _ in 0..(256-80) {  // part 2
        state.simulate_day();
    }
    dbg!(&state, state.total());
    assert_eq!(state.total(), 1728611055389);
}
