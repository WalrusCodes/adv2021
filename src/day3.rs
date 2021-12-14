fn count_ones_in_pos(lines: &[&str], pos: usize) -> usize {
    lines
        .iter()
        .map(|line| {
            if line.chars().nth(pos).unwrap() == '1' {
                1
            } else {
                0
            }
        })
        .sum()
}

fn keep_with_value_in_pos<'a>(lines: &[&'a str], pos: usize, value: char) -> Vec<&'a str> {
    lines
        .iter()
        .filter(|&&line| line.chars().nth(pos).unwrap() == value)
        .copied()
        .collect()
}

fn part1(lines: &[&str]) {
    let bit_count = lines[0].len();
    dbg!(&bit_count);
    let mut gamma = 0usize;
    for pos in 0..bit_count {
        let cnt_ones = count_ones_in_pos(lines, pos);
        gamma <<= 1;
        if cnt_ones > (lines.len() / 2) {
            gamma += 1;
        }
        dbg!(&pos, &cnt_ones, &gamma);
    }
    let epsilon = 2usize.pow(bit_count as u32) - 1 - gamma;
    dbg!(&epsilon);
    dbg!(epsilon * gamma);
}

fn filter_by_pos<'a>(lines_in: &[&'a str], keep_most_common: bool) -> u32 {
    let mut lines = lines_in.to_vec();
    let bit_count = lines[0].len();
    for pos in 0..bit_count {
        dbg!(&lines, &pos);
        let cnt_ones = count_ones_in_pos(&lines, pos);
        let cnt_zeroes = lines.len() - cnt_ones;
        dbg!(&cnt_ones, &cnt_zeroes);
        let keep_char = if keep_most_common {
            if cnt_ones >= cnt_zeroes {
                '1'
            } else {
                '0'
            }
        } else {
            // keep least common, breaking ties towards 0
            if (cnt_ones == 0) || ((cnt_zeroes > 0) && (cnt_zeroes <= cnt_ones)) {
                '0'
            } else {
                '1'
            }
        };
        lines = keep_with_value_in_pos(&lines, pos, keep_char);
    }
    dbg!(&lines);
    assert_eq!(lines.len(), 1);
    u32::from_str_radix(lines[0], 2).unwrap()
}

fn part2(lines_in: &[&str]) {
    let oxygen = dbg!(filter_by_pos(lines_in, true));
    let co2 = dbg!(filter_by_pos(lines_in, false));
    dbg!(oxygen * co2);
}

fn main() {
    let path = std::env::args().nth(1).expect("pls provide input file");
    let contents = std::fs::read_to_string(path).expect("read failed");
    let lines: Vec<_> = contents.lines().collect();
    // part1(&lines);
    part2(&lines);
}
