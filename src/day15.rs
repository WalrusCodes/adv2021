use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Pos(usize, usize);

#[derive(Debug, Clone)]
struct State {
    risk_levels: HashMap<Pos, usize>,
    queue: BinaryHeap<Reverse<(usize, Pos)>>,
    best: HashMap<Pos, usize>,
}

impl State {
    fn parse(lines: &str) -> State {
        let mut risk_levels = HashMap::new();
        lines.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                risk_levels.insert(Pos(x, y), c.to_digit(10).unwrap() as usize);
            })
        });
        let mut queue = BinaryHeap::new();
        queue.push(Reverse((0, Pos(0, 0))));
        let mut best = HashMap::new();
        best.insert(Pos(0, 0), 0);
        State {
            risk_levels,
            queue,
            best,
        }
    }

    fn get_max_pos(&self) -> Pos {
        let max_x = self.risk_levels.keys().map(|&Pos(x, _)| x).max().unwrap();
        let max_y = self.risk_levels.keys().map(|&Pos(_, y)| y).max().unwrap();
        Pos(max_x, max_y)
    }

    fn find_path(&mut self) -> usize {
        let Pos(max_x, max_y) = self.get_max_pos();

        while let Some(Reverse((cost, Pos(x, y)))) = self.queue.pop() {
            let risk_level = self.best[&Pos(x, y)];
            if (x, y) == (max_x, max_y) {
                return risk_level;
            }
            if cost > risk_level {
                continue;
            }
            let mut neighbors: Vec<Pos> = Vec::new();
            if x != 0 {
                neighbors.push(Pos(x - 1, y));
            }
            if x != max_x {
                neighbors.push(Pos(x + 1, y));
            }
            if y != 0 {
                neighbors.push(Pos(x, y - 1));
            }
            if y != max_y {
                neighbors.push(Pos(x, y + 1));
            }
            for n in neighbors {
                let new_risk_level = risk_level + self.risk_levels[&n];
                if new_risk_level < *self.best.get(&n).unwrap_or(&usize::MAX) {
                    self.best.insert(n, new_risk_level);
                    self.queue.push(Reverse((new_risk_level, n)));
                }
            }
        }
        panic!("shouldn't happen");
    }

    fn embiggen_for_part2(&mut self) {
        let Pos(max_x, max_y) = self.get_max_pos();
        for x in 0..=max_x {
            for y in 0..=max_y {
                let risk_level = self.risk_levels[&Pos(x, y)];
                for xx in 0..5 {
                    for yy in 0..5 {
                        if xx == 0 && yy == 0 {
                            continue;
                        }
                        let new_pos = Pos(x + xx * (max_x + 1), y + yy * (max_y + 1));
                        let mut new_risk_level = risk_level + xx + yy;
                        if new_risk_level > 9 {
                            new_risk_level -= 9;
                        }
                        self.risk_levels.insert(new_pos, new_risk_level);
                    }
                }
            }
        }
    }
}

fn main() {
    let path = std::env::args().nth(1).expect("pls provide input file");
    let contents = std::fs::read_to_string(path).expect("read failed");

    let mut state = State::parse(&contents);
    // dbg!(&state);
    let mut state2 = state.clone();
    state2.embiggen_for_part2();
    dbg!(state.find_path());
    // dbg!(&state2);
    dbg!(state2.find_path());
}
