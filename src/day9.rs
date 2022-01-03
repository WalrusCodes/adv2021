use std::collections::{HashSet, VecDeque};

fn part1(grid: &Vec<Vec<usize>>) {
    let row_count = grid.len();
    let col_count = grid[0].len();

    let mut risk_level_sum = 0usize;

    for row in 0..row_count {
        for col in 0..col_count {
            let mut cmp: Vec<usize> = Vec::new();
            if row != 0 {
                cmp.push(grid[row - 1][col]);
            }
            if row != (row_count - 1) {
                cmp.push(grid[row + 1][col]);
            }
            if col != 0 {
                cmp.push(grid[row][col - 1]);
            }
            if col != (col_count - 1) {
                cmp.push(grid[row][col + 1]);
            }
            if grid[row][col] < cmp.into_iter().min().unwrap() {
                risk_level_sum += grid[row][col] + 1;
            }
        }
    }
    dbg!(risk_level_sum);
}

fn part2(grid: &Vec<Vec<usize>>) -> usize {
    // (row, col)
    let mut seen = HashSet::<(usize, usize)>::new();
    let mut sizes = Vec::<usize>::new();

    let row_count = grid.len();
    let col_count = grid[0].len();

    for row in 0..row_count {
        for col in 0..col_count {
            if seen.contains(&(row, col)) {
                continue;
            }
            seen.insert((row, col));

            let mut queue = VecDeque::<(usize, usize)>::new();
            queue.push_back((row, col));
            let mut size = 0;
            while !queue.is_empty() {
                let (row, col) = queue.pop_front().unwrap();
                if grid[row][col] == 9 {
                    continue;
                }
                size += 1;
                let mut consider: Vec<(usize, usize)> = Vec::new();
                if row != 0 {
                    consider.push((row - 1, col));
                }
                if row != (row_count - 1) {
                    consider.push((row + 1, col));
                }
                if col != 0 {
                    consider.push((row, col - 1));
                }
                if col != (col_count - 1) {
                    consider.push((row, col + 1));
                }
                for row_col in consider {
                    if !seen.contains(&row_col) {
                        seen.insert(row_col);
                        queue.push_back(row_col);
                    }
                }
            }
            sizes.push(size);
        }
    }
    sizes.sort();
    // dbg!(sizes);
    sizes[sizes.len() - 1] * sizes[sizes.len() - 2] * sizes[sizes.len() - 3]
}

fn main() {
    let path = std::env::args().nth(1).expect("pls provide input file");
    let contents = std::fs::read_to_string(path).expect("read failed");

    let grid: Vec<_> = contents
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect();

    part1(&grid);
    dbg!(part2(&grid));
}
