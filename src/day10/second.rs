use std::{collections::HashSet, fmt::Display, usize};

struct Matrix {
    matrix: Vec<Vec<i32>>,
}

#[derive(Clone, PartialEq, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn dir(&self) -> (i32, i32) {
        match self {
            Direction::Up => (1, 0),
            Direction::Down => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Left => (0, -1),
        }
    }
}

impl Matrix {
    pub fn new(matrix: Vec<Vec<i32>>) -> Self {
        Self { matrix }
    }

    fn iter<F>(&self, mut iter: F)
    where
        F: FnMut(usize, usize, i32),
    {
        for (i, line) in self.matrix.iter().enumerate() {
            for (j, cell) in line.iter().enumerate() {
                iter(i, j, *cell);
            }
        }
    }

    fn get_counts(&mut self, zero: (usize, usize), start: (usize, usize)) -> i32 {
        let pos = start.clone();

        let all = vec![
            Direction::Up,
            Direction::Down,
            Direction::Right,
            Direction::Left,
        ];

        let sum = all
            .iter()
            .map(|dir| {
                let next = (pos.0 as i32 + dir.dir().0, pos.1 as i32 + dir.dir().1);

                if !self.inclides(next) {
                    return 0;
                }

                let next_cell = self.matrix[next.0 as usize][next.1 as usize];
                let curr_cell = self.matrix[pos.0][pos.1];

                if next_cell - curr_cell == 1 {
                    if next_cell == 9 {
                        return 1;
                    }

                    let pos = (next.0 as usize, next.1 as usize);
                    return self.get_counts(zero, pos);
                }

                return 0;
            })
            .sum();

        sum
    }

    pub fn inclides(&self, point: (i32, i32)) -> bool {
        point.0 >= 0
            && point.1 >= 0
            && point.0 < self.matrix.len() as i32
            && point.1 < self.matrix[0].len() as i32
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = write!(f, "Matrix:\n");
        for line in self.matrix.iter() {
            for cell in line.iter() {
                let _ = write!(f, " {} ", cell);
            }
            let _ = write!(f, "\n");
        }
        Ok(())
    }
}

pub fn run() -> i32 {
    let content = std::fs::read_to_string("./inputs/day10.txt").unwrap();
    let matrix = content
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<_>>>();

    let mut matrix = Matrix::new(matrix);

    let mut starts: HashSet<(usize, usize)> = HashSet::new();

    matrix.iter(|y, x, cell| {
        if cell == 0 {
            starts.insert((y, x));
        }
    });

    let mut sum = 0;
    starts.iter().for_each(|start| {
        let count = matrix.get_counts(*start, *start);
        //println!("Start: {:?}, Count: {}", start, count);
        sum += count;
    });

    //println!("M: {}", matrix);

    sum
}
