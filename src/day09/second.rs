use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
enum DiskState {
    Empty,
    Value(i32),
}

impl DiskState {
    fn is_empty(&self) -> bool {
        match self {
            DiskState::Empty => true,
            _ => false,
        }
    }

    fn is_value(&self) -> bool {
        match self {
            DiskState::Value(_) => true,
            _ => false,
        }
    }

    pub fn is_different(&self, other: DiskState) -> bool {
        match self {
            DiskState::Value(v) => match other {
                DiskState::Value(v2) => *v != v2,
                _ => true,
            },
            DiskState::Empty if other.is_value() => true,
            _ => false,
        }
    }
}

struct Disk(Vec<DiskState>);

fn get_empty_space(disk: &Vec<DiskState>, len: usize, max_len: usize) -> Option<usize> {
    let mut prev = 0;
    for i in 0..max_len {
        if !disk[i].is_empty() {
            prev = i;
        } else {
            if i - (prev + 1) == len {
                return Some(prev + 1);
            }
        }
    }

    None
}

fn sort(disk: &Vec<DiskState>) -> Vec<DiskState> {
    let old_disk = disk.clone();
    let mut disk = disk.clone();

    let mut i = disk.len() - 1;
    let mut start_index = i;

    while i > 0 {
        let start = disk[start_index];
        let curr = disk[i];
        let next = old_disk[i - 1];
        let old_curr = old_disk[i];

        if curr.is_different(start) {
            start_index = i;
        }

        if (next).is_different(old_curr) {
            if let Some(empty_index) = get_empty_space(&disk, start_index - i, i) {
                for j in 0..(start_index - i + 1) {
                    disk.swap(i + j, empty_index + j);
                }
            }
        }

        i -= 1;
    }

    disk
}

impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for v in &self.0 {
            let v = match v {
                DiskState::Empty => ".",
                DiskState::Value(v) => {
                    let value = format!("({})", v).to_string();

                    &value.to_string()
                }
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

    //println!("{}", Disk(disk.clone()));
    //println!("{}", Disk(sorted));

    sum
}
