fn main() {
    let path = std::env::args().nth(1).expect("pls provide input file");
    let contents = std::fs::read_to_string(path).expect("read failed");
    let mut nums = contents.lines().map(|line| line.parse::<u32>().unwrap());

    let mut count = 0;
    let mut previous = nums.next().unwrap();
    for i in nums {
        if i > previous {
            count += 1;
        }
        previous = i;
    }
    dbg!(count);
}
