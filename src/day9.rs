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
