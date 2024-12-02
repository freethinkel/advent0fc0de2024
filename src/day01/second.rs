use std::collections::HashMap;

use anyhow::Result;

pub fn main() -> Result<()> {
    let content = std::fs::read_to_string("./inputs/day01.txt")?;
    let content = content.trim();

    let data: Vec<Vec<i32>> = content
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|item| item.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let mut left: Vec<i32> = data.iter().map(|pair| pair[0]).collect();
    let mut right: Vec<i32> = data.iter().map(|pair| pair[1]).collect();

    let mut map = HashMap::<i32, i32>::new();

    left.sort();
    right.sort();

    let mut sum = 0;

    for i in 0..left.len() {
        *map.entry(left[i]).or_insert(0) += right.iter().filter(|e| *e == &left[i]).count() as i32;
    }

    map.iter().for_each(|(key, value)| {
        sum += key * value;
    });

    println!("ANSWER: {:?}", sum);

    Ok(())
}
