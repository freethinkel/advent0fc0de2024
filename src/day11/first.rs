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

pub fn run() -> i32 {
    let content = std::fs::read_to_string("./inputs/day11.txt").unwrap();
    let mut stones: Vec<i64> = content
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    for _ in 0..25 {
        let mut new_stones = vec![];

        for j in 0..stones.len() {
            let stone = stones[j];
            let len = get_number_len(stone);
            match stone {
                0 => {
                    new_stones.push(1);
                }
                n if len % 2 == 0 => {
                    let (a, b) = get_number_pairs(len, n);
                    new_stones.push(a);
                    new_stones.push(b);
                }
                s => {
                    new_stones.push(s * 2024);
                }
            }
        }
        //println!("{:?}", new_stones);

        stones = new_stones;
    }

    stones.len() as i32
}
