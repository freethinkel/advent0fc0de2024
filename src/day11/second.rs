use std::collections::HashMap;

fn get_number_len(n: i64) -> usize {
    let mut n = n;
    let mut len = 0;
    while n > 0 {
        n /= 10;
        len += 1;
    }

    len
}

fn get_number_pairs(len: usize, n: i64) -> (i64, i64) {
    let mut b = n;
    for _ in 0..(len / 2) {
        b = b / 10;
    }

    let a = n % (b * 10_i64.pow(len as u32 / 2));

    (b, a)
}

pub fn run() -> i64 {
    let content = std::fs::read_to_string("./inputs/day11.txt").unwrap();
    let stones: Vec<i64> = content
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut counts = HashMap::<i64, i64>::new();

    for j in 0..stones.len() {
        let stone = stones[j];
        *counts.entry(stone).or_default() += 1;
    }
    for _ in 0..75 {
        let mut next = HashMap::new();
        for (stone, count) in counts {
            let len = get_number_len(stone);
            match stone {
                0 => {
                    *next.entry(1).or_default() += count;
                }
                n if len % 2 == 0 => {
                    let (a, b) = get_number_pairs(len, n);
                    *next.entry(a).or_default() += count;
                    *next.entry(b).or_default() += count;
                }
                s => {
                    *next.entry(s * 2024).or_default() += count;
                }
            }
        }
        counts = next;
    }

    counts.values().sum::<i64>()
}
