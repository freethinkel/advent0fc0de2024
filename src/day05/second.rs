fn prepare_rules(rules: &str) -> Vec<(i32, i32)> {
    let rules = rules.split("\n").collect::<Vec<&str>>();
    let rules = rules
        .iter()
        .map(|line| {
            let parts = line.split("|").collect::<Vec<&str>>();
            let key = parts[0].parse::<i32>().unwrap();
            let value = parts[1].parse::<i32>().unwrap();
            (key, value)
        })
        .collect::<Vec<(i32, i32)>>();

    rules
}

fn prepare_seq(seq: &str) -> Vec<Vec<i32>> {
    let seq = seq
        .split("\n")
        .into_iter()
        .map(|line| {
            let line = line
                .split(",")
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            line
        })
        .collect::<Vec<Vec<i32>>>();
    seq
}

fn get_keys_from_value(value: i32, rules: &Vec<(i32, i32)>) -> Vec<i32> {
    let mut keys = vec![];

    for rule in rules {
        if rule.1 == value {
            keys.push(rule.0);
        }
    }

    keys
}

fn is_line_vaid(line: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
    for (i, n) in line.iter().enumerate() {
        let keys = get_keys_from_value(*n, rules);
        let is_contains = line[i..].iter().any(|nn| keys.contains(nn));

        if is_contains {
            return false;
        }
    }

    true
}

fn sort(line: &Vec<i32>, rules: &Vec<(i32, i32)>) -> Vec<i32> {
    let mut new_line = line.clone();
    for i in 0..new_line.len() {
        let n = &new_line[i];

        let keys = get_keys_from_value(*n, rules);
        let bad_number = new_line[i..].iter().find(|nn| keys.contains(nn));

        if let Some(bad_number) = bad_number {
            let j = new_line.iter().position(|x| x == bad_number).unwrap();
            new_line.swap(i, j);
        }
    }

    if !is_line_vaid(&new_line, rules) {
        new_line = sort(&new_line, rules);
    }

    new_line
}

pub fn run() {
    let content = std::fs::read_to_string("./inputs/day05.txt").unwrap();
    let parts = content.split("\n\n").collect::<Vec<&str>>();
    let rules = prepare_rules(parts[0]);
    let seq = prepare_seq(parts[1].trim());

    let mut sum = 0;

    for line in seq {
        let is_valid = is_line_vaid(&line, &rules);
        if !is_valid {
            let line = line.clone();
            let line = sort(&line, &rules);
            let center = line[line.len() / 2];
            sum += center;
        }
    }

    println!("ANSWER: {:?}", sum);
}
