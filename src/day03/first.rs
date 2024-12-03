pub fn run() {
    let content = std::fs::read_to_string("inputs/day03.txt").unwrap();

    let reg = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut sum = 0;
    reg.captures_iter(&content).for_each(|c| {
        let (_, [left, right]) = c.extract();
        let left = left.parse::<i32>().unwrap();
        let right = right.parse::<i32>().unwrap();

        sum += left * right;
    });

    println!("ANSWER: {}", sum);
}
