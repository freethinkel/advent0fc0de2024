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
        let variants = Operation::generate(calc.numbers.len() - 1);

        let is_possible = calc.possible(variants);
        if is_possible {
            sum += calc.answer;
        }
    });

    sum
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Operation {
    Add,
    Multiply,
    Concat,
}

impl Operation {
    pub fn generate(len: usize) -> Vec<Vec<Operation>> {
        let variants = [Operation::Add, Operation::Multiply, Operation::Concat];
        let mut combinations = Vec::new();

        for i in 0..variants.len().pow(len as u32) {
            let mut operations = Vec::new();
            let mut num = i;

            for _ in 0..len {
                let op = variants[num % variants.len()];
                operations.push(op);
                num /= variants.len();
            }

            operations.reverse();
            combinations.push(operations);
        }

        combinations
    }
}

#[derive(Debug, Default)]
struct Calculator {
    answer: i64,
    numbers: Vec<i64>,
}

impl Calculator {
    fn get_sum(&self, operations: Vec<Operation>) -> i64 {
        let mut index = 1;
        let operations = operations.iter().collect::<Vec<&Operation>>();
        let numbers = self.numbers.clone();
        let mut sum = numbers[0].clone();

        if numbers.len() == 1 || sum == self.answer {
            return sum;
        }

        for operation in operations {
            match operation {
                Operation::Add => {
                    sum += numbers[index];
                }
                Operation::Multiply => {
                    sum *= numbers[index];
                }
                Operation::Concat => {
                    let mut out = sum.clone();
                    let mut tmp = numbers[index].clone();
                    while tmp > 0 {
                        tmp /= 10;
                        out *= 10;
                    }
                    sum = out + numbers[index];
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
