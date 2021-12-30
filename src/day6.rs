#[derive(Debug)]
struct State {
    counts: [usize; 9],
}

impl State {
    fn parse(line: &str) -> State {
        let mut counts = [0usize; 9];
        for fish_str in line.split(",") {
            let idx = fish_str.parse::<usize>().unwrap();
            counts[idx] += 1;
        }

        State { counts }
    }
    fn simulate_day(&mut self) {
        let zero = self.counts[0];
        for idx in 0..8 {
            self.counts[idx] = self.counts[idx + 1];
        }
        self.counts[8] = zero;
        self.counts[6] += zero;
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
    for _ in 0..(256-80) {  // part 2
        state.simulate_day();
    }
    dbg!(&state, state.total());
}
