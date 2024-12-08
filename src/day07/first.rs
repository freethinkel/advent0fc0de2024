pub fn run() -> i64 {
    let content = std::fs::read_to_string("./inputs/day07.txt").unwrap();
    let lines = content.trim().lines();
    let calcs = lines
        .map(|line| {
            let parts = line.split(":").collect::<Vec<&str>>();
            let answer = parts[0].trim().parse::<i64>().unwrap();
            let numbers = parts[1]
                .trim()
                .split_whitespace()
                .map(|x| x.trim().parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            Calculator { answer, numbers }
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    calcs.iter().for_each(|calc| {
        let variants = Operation::generate((calc.numbers.len() - 1) as i32);
        let is_possible = calc.possible(variants);
        if is_possible {
            sum += calc.answer;
        }
    });

    sum
}

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

impl Operation {
    pub fn generate(len: i32) -> Vec<Vec<Operation>> {
        let mut variants = vec![];

        for i in 0..(1 << len) {
            let mut operations = vec![];
            for j in 0..len {
                if i & (1 << j) != 0 {
                    operations.push(Operation::Add);
                } else {
                    operations.push(Operation::Multiply);
                }
            }
            variants.push(operations);
        }

        variants
    }
}

#[derive(Debug, Default)]
struct Calculator {
    answer: i64,
    numbers: Vec<i64>,
}

impl Calculator {
    fn get_sum(&self, operations: Vec<Operation>) -> i64 {
        let mut sum = self.numbers[0];
        let mut index = 1;

        for operation in operations {
            self.numbers[index];
            match operation {
                Operation::Add => {
                    sum += self.numbers[index];
                }
                Operation::Multiply => {
                    sum *= self.numbers[index];
                }
            }
            index += 1;
        }

        sum
    }

    fn possible(&self, variants: Vec<Vec<Operation>>) -> bool {
        for variant in variants {
            let sum = self.get_sum(variant);
            if sum == self.answer {
                return true;
            }
        }

        return false;
    }
}
