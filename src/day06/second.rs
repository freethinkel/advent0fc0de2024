use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Empty,
    Obstacle,
    Player(Direction),
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

struct Map {
    matrix: Vec<Vec<Cell>>,
}

fn is_overlfow<T>(point: &(i32, i32), matrix: &Vec<Vec<T>>) -> bool {
    let (y, x) = point;
    return !(y < &(matrix.len() as i32) && y >= &0 && x >= &0 && x < &(matrix[0].len() as i32));
}

impl Map {
    fn get_player_position(&self) -> (usize, usize) {
        let pos = &self
            .matrix
            .iter()
            .enumerate()
            .find_map(|(i, line)| {
                let pos = line.iter().position(|cell| match cell {
                    Cell::Player(_) => true,
                    _ => false,
                });

                if let Some(pos) = pos {
                    return Some((i, pos));
                }

                None
            })
            .unwrap();

        return pos.clone();
    }

    pub fn visited(&self) -> HashSet<(usize, usize)> {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut pos = self.get_player_position();
        let mut dir = Direction::Up;

        loop {
            let (y, x) = pos;
            visited.insert(pos);

            let next = match dir {
                Direction::Up => (y as i32 - 1, x as i32),
                Direction::Down => (y as i32 + 1, x as i32),
                Direction::Right => (y as i32, x as i32 + 1),
                Direction::Left => (y as i32, x as i32 - 1),
            };

            if is_overlfow(&next, &self.matrix) {
                break;
            }
            let new_cell = self.matrix[next.0 as usize][next.1 as usize];
            if new_cell == Cell::Obstacle {
                dir = dir.turn_right();
            } else {
                pos = (next.0 as usize, next.1 as usize)
            }
        }

        visited
    }

    pub fn is_stuck(&self, obstacle: (usize, usize)) -> bool {
        let mut matrix = self.matrix.clone();
        matrix[obstacle.0][obstacle.1] = Cell::Obstacle;
        let mut visited: HashSet<((usize, usize), Direction)> = HashSet::new();
        let mut pos = self.get_player_position();
        let mut dir = Direction::Up;

        if obstacle == pos {
            return false;
        }

        loop {
            let (y, x) = pos;

            let next = match dir {
                Direction::Up => (y as i32 - 1, x as i32),
                Direction::Down => (y as i32 + 1, x as i32),
                Direction::Right => (y as i32, x as i32 + 1),
                Direction::Left => (y as i32, x as i32 - 1),
            };

            if is_overlfow(&next, &self.matrix) {
                break;
            }

            let next = (next.0 as usize, next.1 as usize);

            if !visited.insert((next, dir)) {
                return true;
            }
            let new_cell = matrix[next.0][next.1];
            if new_cell == Cell::Obstacle {
                dir = dir.turn_right();
            } else {
                pos = next
            }
        }

        false
    }
}

pub fn run() -> i32 {
    let content = std::fs::read_to_string("./inputs/day06.txt").unwrap();

    let lines = content
        .trim()
        .lines()
        .map(|line| line.split("").collect::<Vec<&str>>()[1..(line.len() + 1)].to_vec())
        .collect::<Vec<Vec<&str>>>();

    let matrix: Vec<Vec<Cell>> = lines
        .iter()
        .map(|line| {
            line.iter()
                .map(|&char| match char {
                    "^" => Cell::Player(Direction::Up),
                    "#" => Cell::Obstacle,
                    _ => Cell::Empty,
                })
                .collect::<Vec<Cell>>()
        })
        .collect();
    let map = Map { matrix };
    let visited = map.visited();

    let sum = visited
        .iter()
        .enumerate()
        .filter(|(i, pos)| {
            println!("I: {}", i);

            map.is_stuck(**pos)
        })
        .count();

    sum as i32
}
