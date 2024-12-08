use std::collections::{HashMap, HashSet};

pub fn run() -> i32 {
    let content = std::fs::read_to_string("inputs/day08.txt").expect("reading input file");

    let lines = content.lines();

    let matrix: Vec<Vec<char>> = lines
        .into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (y, row) in matrix.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            match col {
                '.' => {}
                col => {
                    antennas.entry(*col).or_insert(Vec::new());

                    antennas.get_mut(col).unwrap().push((y, x));
                }
            }
        }
    }

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for (_, coords) in antennas.iter() {
        for coord in coords {
            for other_coord in coords {
                if coord != other_coord {
                    let coord = (coord.0 as i32, coord.1 as i32);
                    let other_coord = (other_coord.0 as i32, other_coord.1 as i32);
                    let diff = (other_coord.0 - coord.0, other_coord.1 - coord.1);

                    antinodes.insert((coord.0 - diff.0, coord.1 - diff.1));
                    antinodes.insert((other_coord.0 + diff.0, other_coord.1 + diff.1));
                }
            }
        }
    }

    let antinodes: Vec<(i32, i32)> = antinodes
        .into_iter()
        .filter(|(y, x)| {
            x >= &0 && y >= &0 && *y < matrix.len() as i32 && *x < matrix[0].len() as i32
        })
        .collect();

    antinodes.len() as i32
}
