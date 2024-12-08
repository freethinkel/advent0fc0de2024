use std::collections::{HashMap, HashSet};

pub fn run() -> i32 {
    let content = std::fs::read_to_string("inputs/day08.txt").expect("reading input file");

    let lines = content.lines();

    let matrix: Vec<Vec<char>> = lines
        .into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let matrix = Matrix { matrix };

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

    for (ant, coords) in antennas.iter() {
        for coord in coords {
            if (coords.len() as i32) < 2 {
                continue;
            }
            for other_coord in coords {
                antinodes.insert((coord.0 as i32, coord.1 as i32));
                if coord != other_coord {
                    let coord = (coord.0 as i32, coord.1 as i32);
                    let other_coord = (other_coord.0 as i32, other_coord.1 as i32);
                    let diff = (other_coord.0 - coord.0, other_coord.1 - coord.1);

                    let mut a = (coord.0 - diff.0, coord.1 - diff.1);
                    let mut b = (other_coord.0 + diff.0, other_coord.1 + diff.1);

                    while matrix.inclides(a) {
                        antinodes.insert(a);
                        a.0 -= diff.0;
                        a.1 -= diff.1;
                    }

                    while matrix.inclides(b) {
                        antinodes.insert(b);
                        b.0 += diff.0;
                        b.1 += diff.1;
                    }
                }
            }
        }
    }

    let antinodes: Vec<(i32, i32)> = antinodes.into_iter().collect();

    antinodes.len() as i32
}

struct Matrix<T> {
    matrix: Vec<Vec<T>>,
}

impl<T> Matrix<T> {
    pub fn inclides(&self, point: (i32, i32)) -> bool {
        point.0 >= 0
            && point.1 >= 0
            && point.0 < self.matrix.len() as i32
            && point.1 < self.matrix[0].len() as i32
    }

    pub fn iter(&self) -> std::slice::Iter<Vec<T>> {
        self.matrix.iter()
    }
}
