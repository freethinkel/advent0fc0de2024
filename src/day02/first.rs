use anyhow::Result;

pub fn run() -> Result<()> {
    let content = std::fs::read_to_string("inputs/day02.txt")?;
    let content = content.trim();

    let mut sum = 0;

    for line in content.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|char| char.parse::<i32>().unwrap())
            .collect();

        let mut prev: Option<i32> = None;
        let mut prev_dir = 0;
        let mut valid = true;

        for i in numbers.clone() {
            let mut curr_dir = 0;
            if let Some(prev) = prev {
                if i - prev > 0 {
                    curr_dir = 1;
                } else {
                    curr_dir = -1;
                }
            }
            if prev_dir == 0 {
                prev_dir = curr_dir;
            }

            let is_ok = match prev {
                Some(prev) => (i - prev).abs() < 4 && (i - prev).abs() > 0 && curr_dir == prev_dir,
                _ => true,
            };
            prev_dir = curr_dir;
            prev = Some(i);
            if !is_ok {
                valid = false;
                break;
            }
        }

        if valid {
            sum += 1;
        }
    }

    println!("ANSWER: {:?}", sum);

    Ok(())
}
