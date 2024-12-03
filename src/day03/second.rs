pub fn run() {
    let content = std::fs::read_to_string("inputs/day03.txt").unwrap();

    let reg = regex::Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut sum = 0;
    let mut is_allowed = true;

    reg.captures_iter(&content).for_each(|chunk| {
        let matched = chunk.get(0).unwrap().as_str();
        let _ = match matched {
            "don't()" => is_allowed = false,
            "do()" => is_allowed = true,
            _ => {
                if !is_allowed {
                    return;
                }
                let l = &chunk[1].parse::<i32>().unwrap();
                let r = &chunk[2].parse::<i32>().unwrap();
                sum += l * r;
            }
        };
    });

    println!("ANSWER: {}", sum);
}
