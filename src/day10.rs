fn matching_close(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("invalid char"),
    }
}

fn error_score(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("invalid char"),
    }
}

fn is_corrupted(line: &str) -> Option<u32> {
    let mut v: Vec<char> = Vec::new();
    for c in line.chars() {
        if "([{<".contains(c) {
            v.push(c);
        } else {
            let prev = v.pop().unwrap();
            if matching_close(prev) != c {
                return Some(error_score(c));
            }
        }
    }
    None
}

fn part1(lines: &str) -> u32 {
    lines
        .lines()
        .map(|line| is_corrupted(line).unwrap_or(0))
        .sum()
}

fn completion_score(c: char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("invalid char"),
    }
}

fn complete_line(line: &str) -> Option<u64> {
    let mut v: Vec<char> = Vec::new();
    for c in line.chars() {
        if "([{<".contains(c) {
            v.push(c);
        } else {
            let prev = v.pop().unwrap();
            if matching_close(prev) != c {
                return None;
            }
        }
    }
    let mut score = 0;
    for &c in v.iter().rev() {
        score = score * 5 + completion_score(matching_close(c));
    }
    Some(score)
}

fn part2(lines: &str) -> u64 {
    let mut scores: Vec<u64> = lines
        .lines()
        .filter_map(|line| complete_line(line))
        .collect();
    scores.sort();
    scores[scores.len() / 2]
}

fn main() {
    let path = std::env::args().nth(1).expect("pls provide input file");
    let contents = std::fs::read_to_string(path).expect("read failed");

    dbg!(part1(&contents));
    dbg!(part2(&contents));
}
