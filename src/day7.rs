const MAX: usize = 2000;

#[derive(Debug)]
struct State {
    sum_left: [usize; MAX],
    count_left: [usize; MAX],
    sum_right: [usize; MAX],
    count_right: [usize; MAX],
    count: [usize; MAX],
}

fn read_numbers() -> Vec<usize> {
    let path = std::env::args().nth(1).expect("pls provide input file");
    let contents = std::fs::read_to_string(path).expect("read failed");
    contents
        .trim()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

#[allow(unused)]
fn part1(state: &mut State) {
    // fill in {sum,count}_left
    for idx in 1..MAX {
        state.count_left[idx] = state.count_left[idx - 1] + state.count[idx - 1];
        state.sum_left[idx] =
            state.sum_left[idx - 1] + state.count[idx - 1] + state.count_left[idx - 1];
    }
    // fill in {sum,count}_right
    for idx in (0..(MAX - 1)).rev() {
        state.count_right[idx] = state.count_right[idx + 1] + state.count[idx + 1];
        state.sum_right[idx] =
            state.sum_right[idx + 1] + state.count[idx + 1] + state.count_right[idx + 1];
    }
}

fn cost(from: usize, to: usize) -> usize {
    if from > to {
        cost(to, from)
    } else {
        let diff = to - from;
        (diff * diff + diff) / 2
    }
}

#[allow(unused)]
fn part2(state: &mut State) {
    for idx in 0..MAX {
        if state.count[idx] == 0 {
            continue;
        }
        for idx2 in 0..MAX {
            state.sum_left[idx2] += state.count[idx] * cost(idx, idx2);
        }
    }
}

fn main() {
    let mut state = State {
        sum_left: [0; MAX],
        count_left: [0; MAX],
        sum_right: [0; MAX],
        count_right: [0; MAX],
        count: [0; MAX],
    };
    let mut crab_count = 0usize;
    for &num in read_numbers().iter() {
        state.count[num] += 1;
        crab_count += 1;
    }
    dbg!(crab_count);
    // part1(&mut state);
    part2(&mut state);
    let min = (0..MAX)
        .map(|idx| state.sum_left[idx] + state.sum_right[idx])
        .min()
        .unwrap();
    dbg!(&min);
}
