fn main() {
    let path = std::env::args().nth(1).expect("pls provide input file");
    let contents = std::fs::read_to_string(path).expect("read failed");
    let nums: Vec<u32> = contents
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();

    let mut count = 0;
    for start in 0..(nums.len() - 3) {
        dbg!(nums[start], nums[start + 1], nums[start + 2]);
        let previous = nums[start] + nums[start + 1] + nums[start + 2];
        let current = nums[start + 1] + nums[start + 2] + nums[start + 3];
        if current > previous {
            count += 1;
        }
    }

    dbg!(count);
}
