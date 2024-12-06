use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Empty,
    Obstacle,
    Player(Direction),
}

fn get_player_position(matrix: &Vec<Vec<Cell>>) -> (usize, usize) {
    let pos = &matrix
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

fn swap(from: (usize, usize), to: (usize, usize), vec: &mut Vec<Vec<Cell>>) {
    let temp = vec[from.0][from.1];
    vec[from.0][from.1] = vec[to.0][to.1];
    vec[to.0][to.1] = temp;
}

fn is_overlfow<T>(point: &(i32, i32), matrix: &Vec<Vec<T>>) -> bool {
    let (y, x) = point;
    return !(y < &(matrix.len() as i32) && y >= &0 && x >= &0 && x < &(matrix[0].len() as i32));
}

pub fn run() -> i32 {
    let content = std::fs::read_to_string("./inputs/day06.txt").unwrap();

    let lines = content
        .trim()
        .lines()
        .map(|line| line.split("").collect::<Vec<&str>>()[1..(line.len() + 1)].to_vec())
        .collect::<Vec<Vec<&str>>>();

    let mut matrix: Vec<Vec<Cell>> = lines
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

    let mut paths: HashMap<(usize, usize), i32> = HashMap::new();

    loop {
        let player_position = get_player_position(&matrix);
        let (y, x) = player_position;
        paths.insert((y, x), 0);
        let player = &matrix[y][x];
        let to = match player {
            Cell::Player(player) => match player {
                &Direction::Up => (y as i32 - 1, x as i32),
                &Direction::Down => (y as i32 + 1, x as i32),
                &Direction::Right => (y as i32, x as i32 + 1),
                &Direction::Left => (y as i32, x as i32 - 1),
            },
            _ => (0, 0),
        };
        if is_overlfow(&to, &matrix) {
            break;
        }

        let to = (to.0 as usize, to.1 as usize);
        let to_cell = &matrix[to.0][to.1];
        let (to, player) = match to_cell {
            Cell::Obstacle => match player {
                Cell::Player(player) => match player {
                    &Direction::Up => ((y, x + 1), Cell::Player(Direction::Right)),
                    &Direction::Down => ((y, x - 1), Cell::Player(Direction::Left)),
                    &Direction::Right => ((y + 1, x), Cell::Player(Direction::Down)),
                    &Direction::Left => ((y - 1, x), Cell::Player(Direction::Up)),
                },
                _ => (to, player.clone()),
            },
            _ => (to, player.clone()),
        };

        if is_overlfow(&(to.0 as i32, to.1 as i32), &matrix) {
            break;
        }

        matrix[y][x] = player;

        match player {
            Cell::Player(player) => match player {
                Direction::Up => swap(player_position, to, &mut matrix),
                Direction::Down => swap(player_position, to, &mut matrix),
                Direction::Right => swap(player_position, to, &mut matrix),
                Direction::Left => swap(player_position, to, &mut matrix),
            },
            _ => {}
        }
    }

    paths.len() as i32
}
