#[derive(Default, Debug, Clone)]
struct Submarine {
    pos: i32,
    depth: i32,
    aim: i32, // part 2 only
}

impl Submarine {
    fn step_part1(&mut self, line: &str) {
        let items: Vec<_> = line.split(" ").collect();
        assert_eq!(items.len(), 2);
        let count: i32 = items[1].parse().unwrap();
        match items[0] {
            "forward" => self.pos += count,
            "down" => self.depth += count,
            "up" => self.depth -= count,
            _ => panic!("invalid value {}", items[0]),
        }
    }

    fn step_part2(&mut self, line: &str) {
        let items: Vec<_> = line.split(" ").collect();
        assert_eq!(items.len(), 2);
        let count: i32 = items[1].parse().unwrap();
        match items[0] {
            "forward" => {
                self.pos += count;
                self.depth += count * self.aim;
            }
            "down" => self.aim += count,
            "up" => self.aim -= count,
            _ => panic!("invalid value {}", items[0]),
        }
    }
}

fn main() {
    let path = std::env::args().nth(1).expect("pls provide input file");
    let contents = std::fs::read_to_string(path).expect("read failed");

    {
        let mut sub = Submarine::default();
        contents.lines().for_each(|line| sub.step_part1(line));
        dbg!(&sub, sub.pos * sub.depth);
    }
    {
        let mut sub = Submarine::default();
        contents.lines().for_each(|line| sub.step_part2(line));
        dbg!(&sub, sub.pos * sub.depth);
    }
}
