use std::collections::HashSet;

type Grid = HashSet<(isize, isize)>;

#[derive(Debug)]
enum Fold {
    X(isize),
    Y(isize),
}

#[derive(Debug)]
struct Input {
    grid: Grid,
    folds: Vec<Fold>,
}

impl Input {
    fn parse(lines: &str) -> Input {
        let mut grid = Grid::new();
        let mut folds = Vec::new();

        for line in lines.lines() {
            if line.contains(',') {
                let mut it = line.split(',');
                let xy = (
                    it.next().unwrap().parse().unwrap(),
                    it.next().unwrap().parse().unwrap(),
                );
                grid.insert(xy);
            } else if !line.is_empty() {
                assert!(line.starts_with("fold along "));
                let num: isize = line[13..].parse().unwrap();
                folds.push(match line.chars().nth(11).unwrap() {
                    'y' => Fold::Y(num),
                    'x' => Fold::X(num),
                    _ => panic!("unexpected char"),
                });
            }
        }

        Input { grid, folds }
    }
}

fn fold_grid(from: &Grid, fold: &Fold) -> Grid {
    let mut new_grid = Grid::new();
    for &(x, y) in from.iter() {
        match fold {
            &Fold::X(fold_x) => {
                if x < fold_x {
                    new_grid.insert((x, y));
                } else if x > fold_x {
                    new_grid.insert((2 * fold_x - x, y));
                }
            }
            &Fold::Y(fold_y) => {
                if y < fold_y {
                    new_grid.insert((x, y));
                } else if y > fold_y {
                    new_grid.insert((x, 2 * fold_y - y));
                }
            }
        };
    }
    new_grid
}

fn display(grid: &Grid) {
    let max_x = grid.iter().map(|&(x, _)| x).max().unwrap();
    let max_y = grid.iter().map(|&(_, y)| y).max().unwrap();
    for y in 0..=max_y {
        let mut s = String::new();
        for x in 0..=max_x {
            s.push(if grid.contains(&(x, y)) { '#' } else { '.' });
        }
        dbg!(s);
    }
}

fn main() {
    let path = std::env::args().nth(1).expect("pls provide input file");
    let contents = std::fs::read_to_string(path).expect("read failed");

    let input = Input::parse(&contents);

    // part 1:
    let grid2 = fold_grid(&input.grid, &input.folds[0]);
    dbg!(grid2.len());

    // part 2:
    let mut grid = input.grid.clone();
    for fold in input.folds.iter() {
        grid = fold_grid(&grid, fold);
    }
    display(&grid);
}
