pub fn run() {
    let content = std::fs::read_to_string("./inputs/day04.txt").unwrap();

    let mut sum = 0;

    let lines: Vec<Vec<&str>> = content
        .trim()
        .lines()
        .map(|line| line.split("").collect::<Vec<&str>>()[1..(line.len() + 1)].to_vec())
        .collect();

    let rows = lines.len();
    let cols = lines[0].len();

    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            let words = vec![
                [lines[i - 1][j - 1], lines[i][j], lines[i + 1][j + 1]],
                [lines[i - 1][j + 1], lines[i][j], lines[i + 1][j - 1]],
            ];
            let count = words
                .iter()
                .map(|word| {
                    let word = word.join("");
                    match word.as_str() {
                        "MAS" => 1,
                        "SAM" => 1,
                        _ => 0,
                    }
                })
                .sum::<i32>();

            if count == 2 {
                sum += 1;
            }
        }
    }

    println!("ANSWER: {sum}");
}
