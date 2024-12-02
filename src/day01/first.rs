use anyhow::Result;

pub fn main() -> Result<()> {
    let content = std::fs::read_to_string("./inputs/day01.txt")?;
    let content = content.trim();

    let data: Vec<Vec<i64>> = content
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|item| item.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    let mut left: Vec<i64> = data.iter().map(|pair| pair[0]).collect();
    let mut right: Vec<i64> = data.iter().map(|pair| pair[1]).collect();

    left.sort();
    right.sort();

    let mut sum = 0;

    for i in 0..left.len() {
        sum += (left[i] - right[i]).abs();
    }

    println!("ANSWER: {}", sum);

    Ok(())
}
