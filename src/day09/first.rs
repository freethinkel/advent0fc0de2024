use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
enum DiskState {
    Empty,
    Value(i32),
}

struct Disk(Vec<DiskState>);

fn sort(disk: &Vec<DiskState>) -> Vec<DiskState> {
    let mut disk = disk.clone();

    let mut last_emty_from_end = disk.len() - 1;
    for i in 0..disk.len() {
        if i >= last_emty_from_end {
            break;
        }
        let curr = disk[i];
        if curr != DiskState::Empty {
            continue;
        }
        while disk[last_emty_from_end] == DiskState::Empty {
            last_emty_from_end -= 1;
        }
        disk.swap(i, last_emty_from_end);
        last_emty_from_end -= 1;
    }

    disk
}

impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for v in &self.0 {
            let v = match v {
                DiskState::Empty => '.',
                DiskState::Value(v) => v.to_string().chars().next().unwrap(),
            };
            write!(f, "{}", v)?;
        }
        Ok(())
    }
}

pub fn run() -> i64 {
    let content = std::fs::read_to_string("inputs/day09.txt").expect("reading input file");
    let instructions = content
        .trim()
        .chars()
        .map(|c| c.to_string().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut disk: Vec<Vec<DiskState>> = vec![];

    for (index, instruction) in instructions.iter().enumerate() {
        if *instruction == 0 {
            continue;
        }
        if index % 2 == 0 {
            disk.push(vec![
                DiskState::Value(index as i32 / 2);
                *instruction as usize
            ]);
        } else {
            disk.push(vec![DiskState::Empty; *instruction as usize]);
        }
    }

    let disk = disk.into_iter().flatten().collect::<Vec<DiskState>>();
    let sorted = sort(&disk);

    let mut sum = 0;

    sorted.iter().enumerate().for_each(|(i, v)| {
        if let DiskState::Value(v) = v {
            sum += i as i64 * *v as i64;
        }
    });

    // println!("{}", Disk(disk.clone()));
    // println!("{}", Disk(sorted));

    sum
}
