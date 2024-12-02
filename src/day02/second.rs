use anyhow::Result;

fn is_line_valid(line: &Vec<i32>) -> bool {
    let mut prev: Option<i32> = None;
    let mut prev_dir = 0;
    let mut valid = true;

    for curr in line {
        let mut curr_dir = 0;
        if let Some(prev) = prev {
            if curr - prev > 0 {
                curr_dir = 1;
            } else {
                curr_dir = -1;
            }
        }
        if prev_dir == 0 {
            prev_dir = curr_dir;
        }

        let is_ok = match prev {
            Some(prev) => {
                (curr - prev).abs() < 4 && (curr - prev).abs() > 0 && curr_dir == prev_dir
            }
            _ => true,
        };
        prev_dir = curr_dir;
        prev = Some(*curr);
        if !is_ok {
            valid = false;
            break;
        }
    }

    return valid;
}

pub fn run() -> Result<()> {
    let content = std::fs::read_to_string("inputs/day02.txt")?;
    let content = content.trim();

    let mut sum = 0;

    for line in content.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|char| char.parse::<i32>().unwrap())
            .collect();

        let mut valid = is_line_valid(&numbers);

        if !valid {
            for i in 0..numbers.len() {
                let mut temp = numbers.clone();
                temp.remove(i);
                if is_line_valid(&temp) {
                    valid = true;
                    break;
                }
            }
        }

        if valid {
            sum += 1;
        }
    }

    println!("ANSWER: {:?}", sum);

    Ok(())
}
