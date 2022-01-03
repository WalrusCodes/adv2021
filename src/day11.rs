use std::collections::VecDeque;

fn step(grid: &mut Vec<Vec<u8>>) -> u32 {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let row_count = grid.len();
    let col_count = grid[0].len();
    for row in 0..row_count {
        for col in 0..col_count {
            queue.push_back((row, col));
        }
    }
    while !queue.is_empty() {
        let (row, col) = queue.pop_front().unwrap();
        if grid[row][col] < 10 {
            grid[row][col] += 1;
            if grid[row][col] == 10 {
                // flash
                if row != 0 {
                    queue.push_back((row - 1, col));
                    if col != 0 {
                        queue.push_back((row - 1, col - 1));
                    }
                    if col != (col_count - 1) {
                        queue.push_back((row - 1, col + 1));
                    }
                }
                if row != (row_count - 1) {
                    queue.push_back((row + 1, col));
                    if col != 0 {
                        queue.push_back((row + 1, col - 1));
                    }
                    if col != (col_count - 1) {
                        queue.push_back((row + 1, col + 1));
                    }
                }
                if col != 0 {
                    queue.push_back((row, col - 1));
                }
                if col != (col_count - 1) {
                    queue.push_back((row, col + 1));
                }
            }
        }
    }
    let mut flashes = 0;
    for row in 0..row_count {
        for col in 0..col_count {
            if grid[row][col] == 10 {
                grid[row][col] = 0;
                flashes += 1;
            }
        }
    }
    flashes
}

fn main() {
    let path = std::env::args().nth(1).expect("pls provide input file");
    let contents = std::fs::read_to_string(path).expect("read failed");

    let mut grid: Vec<_> = contents
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect();
    let mut total_flashes = 0u32;
    for i in 0..1000 {
        let flashes = step(&mut grid);
        total_flashes += flashes;
        if i == 99 {
            dbg!(&total_flashes);
        }
        if flashes as usize == (grid.len() * grid[0].len()) {
            dbg!(i + 1);
            break;
        }
    }
}
