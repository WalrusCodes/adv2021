use std::str::Lines;

#[derive(Debug, Default)]
struct Board {
    numbers: [[u8; 5]; 5],
    seen: [[bool; 5]; 5],
    done: bool,
}

#[derive(Debug)]
struct State {
    numbers: Vec<u8>,
    boards: Vec<Board>,
    boards_done: usize,
}

impl Board {
    fn is_row_done(&self, row: usize) -> bool {
        !(0..5).any(|col| !self.seen[row][col])
    }

    fn is_col_done(&self, col: usize) -> bool {
        !(0..5).any(|row| !self.seen[row][col])
    }

    fn sum_unmarked(&self) -> u32 {
        // self.numbers
        //     .iter()
        //     .zip(self.seen)
        //     .fold(0, |sum, (numbers_row, seen_row)| {
        //         sum + numbers_row
        //             .iter()
        //             .zip(seen_row)
        //             .fold(0, |sum, (&num, seen)| {
        //                 sum + if seen { 0 } else { num as u32 }
        //             })
        //     })

        let mut sum = 0u32;
        for row in 0..5 {
            for col in 0..5 {
                if !self.seen[row][col] {
                    sum += self.numbers[row][col] as u32;
                }
            }
        }
        sum
    }

    fn apply(&mut self, number: u8) -> bool {
        for row in 0..5 {
            for col in 0..5 {
                if self.numbers[row][col] == number {
                    self.seen[row][col] = true;
                    self.done = self.is_row_done(row) || self.is_col_done(col);
                    return self.done;
                }
            }
        }
        false
    }
}

fn parse_called_numbers(line: &str) -> Vec<u8> {
    line.trim().split(",").map(|x| x.parse().unwrap()).collect()
}

fn parse_board(lines: &mut Lines) -> Option<Board> {
    lines.next()?; // skip blank line
    let mut board = Board::default();
    for i in 0..5 {
        let line = lines.next()?;
        for (j, value) in line.split_whitespace().enumerate() {
            board.numbers[i][j] = value.parse().unwrap();
        }
    }
    Some(board)
}

impl State {
    fn parse(contents: &str) -> State {
        let mut lines_iter = contents.lines();
        let numbers = parse_called_numbers(lines_iter.next().unwrap());
        // dbg!(&called_numbers);
        let boards = State::parse_boards(lines_iter);
        // dbg!(&boards);
        State {
            numbers,
            boards,
            boards_done: 0,
        }
    }

    fn parse_boards(mut lines: Lines) -> Vec<Board> {
        let mut boards = Vec::new();
        while let Some(board) = parse_board(&mut lines) {
            boards.push(board);
        }
        boards
    }

    fn play(&mut self) {
        let board_count = self.boards.len();
        for &number in self.numbers.iter() {
            for board in self.boards.iter_mut() {
                if board.done {
                    continue;
                }
                if board.apply(number) {
                    board.done = true;
                    self.boards_done += 1;
                    if self.boards_done == 1 {
                        dbg!(dbg!(board.sum_unmarked()) * dbg!(number as u32));
                    } else if self.boards_done == board_count {
                        dbg!(dbg!(board.sum_unmarked()) * dbg!(number as u32));
                        return;
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
    state.play();
}
