fn xmas_count(line: &Vec<&str>) -> usize {
    let mut count = 0;
    let mut reversed = line.clone();
    reversed.reverse();

    count += line.join("").matches(r"XMAS").count();
    count += reversed.join("").matches(r"XMAS").count();
    count
}

fn rotate_45<T: Clone>(matrix: Vec<Vec<T>>) -> Vec<Vec<Option<T>>> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let new_size = rows + cols - 1;
    let mut rotated = vec![vec![None; new_size]; new_size];

    for i in 0..rows {
        for j in 0..cols {
            let x = ((i as i32 - j as i32) + (cols - 1) as i32) as usize;
            let y = i + j;

            rotated[x][y] = Some(matrix[i][j].clone());
        }
    }

    rotated
}

fn rotate_90<T: Clone>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut rotated = vec![vec![matrix[0][0].clone(); rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            rotated[j][rows - 1 - i] = matrix[i][j].clone();
        }
    }
    rotated
}

pub fn run() {
    let content = std::fs::read_to_string("./inputs/day04.txt").unwrap();

    let mut sum = 0;

    let lines: Vec<Vec<&str>> = content
        .trim()
        .lines()
        .map(|line| line.split("").collect::<Vec<&str>>()[1..(line.len() + 1)].to_vec())
        .collect();

    for (_, line) in lines.iter().enumerate() {
        sum += xmas_count(line);
    }

    let rotated = rotate_90(lines.clone());
    for (_, line) in rotated.iter().enumerate() {
        sum += xmas_count(line);
    }

    let rotated = rotate_45(rotated.clone());
    rotated.iter().for_each(|line| {
        let line: Vec<&str> = line.iter().filter_map(|x| x.as_deref()).collect();
        sum += xmas_count(&line);
    });

    let rotated = rotate_45(lines);
    rotated.iter().for_each(|line| {
        let line: Vec<&str> = line.iter().filter_map(|x| x.as_deref()).collect();
        sum += xmas_count(&line);
    });

    println!("ANSWER: {sum}");
}
