use anyhow::Result;

fn main() -> Result<()> {
    let content = std::fs::read_to_string("./src/input.txt")?;

    let mut vecc: Vec<(i64, i64)> = content
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            println!("parts: {:?}", parts);
            let [left, right] = match parts.as_slice() {
                [left, right] => [left, right],
                _ => panic!("Expected exactly two parts"),
            };

            let left: i64 = left.parse().unwrap();
            let right: i64 = right.parse().unwrap();

            return (left, right);
        })
        .collect();

    let mut left: Vec<&i64> = vecc.iter().map(|(item, _)| item).collect();
    let mut right: Vec<&i64> = vecc.iter().map(|(item, _)| item).collect();
    let size = left.len();
    // while (size > 0) {}
    println!("left: {:?}", left);

    Ok(())
}
