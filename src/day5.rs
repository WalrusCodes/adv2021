struct State {
    used: [[bool; 1000]; 1000],
    overlapping: [[bool; 1000]; 1000],
    overlapping_count: usize,
    ignore_diagonal: bool,
}

#[derive(Debug)]
struct Line {
    x1: isize,
    y1: isize,
    x2: isize,
    y2: isize,
}

impl Line {
    fn parse(s: &str) -> Line {
        // 105,697 -> 287,697
        let v: Vec<_> = s
            .split(|c: char| !c.is_digit(10))
            .filter(|x| !x.is_empty())
            .map(|n| n.parse::<isize>().unwrap())
            .collect();
        match &v[..] {
            &[x1, y1, x2, y2] => Line { x1, y1, x2, y2 },
            _ => panic!(),
        }
    }
}

impl State {
    fn draw_one_line(&mut self, line: &Line) {
        let diff_x = line.x2 - line.x1;
        let diff_y = line.y2 - line.y1;
        if self.ignore_diagonal && diff_x != 0 && diff_y != 0 {
            // part1: ignore non-vertical or horizontal lines
            return;
        }
        let steps = if diff_x.abs() > diff_y.abs() {
            diff_x.abs() + 1
        } else {
            diff_y.abs() + 1
        };
        let dx = diff_x.signum();
        let dy = diff_y.signum();
        let mut x = line.x1 as usize;
        let mut y = line.y1 as usize;
        dbg!(line, x, y, steps, dx, dy);
        for _ in 0..steps {
            if self.used[y][x] {
                if !self.overlapping[y][x] {
                    self.overlapping[y][x] = true;
                    self.overlapping_count += 1;
                }
            } else {
                self.used[y][x] = true;
            }
            x = ((x as isize) + dx) as usize;
            y = ((y as isize) + dy) as usize;
        }
    }
    fn draw_lines(&mut self, lines: &[Line]) {
        for line in lines.iter() {
            self.draw_one_line(line);
        }
    }
}

fn main() {
    let path = std::env::args().nth(1).expect("pls provide input file");
    let contents = std::fs::read_to_string(path).expect("read failed");
    let lines: Vec<_> = contents.lines().map(|l| Line::parse(l)).collect();
    let mut state = State {
        used: [[false; 1000]; 1000],
        overlapping: [[false; 1000]; 1000],
        overlapping_count: 0,
        ignore_diagonal: false,  // part1 or part2
    };
    state.draw_lines(&lines);
    dbg!(state.overlapping_count);
}
