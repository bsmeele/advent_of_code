#[derive(Clone)]
enum Operation {
    Multiply,
    Add,
    Power,
}

#[derive(Clone)]
struct Monkey {
    items: Vec<u32>,
    operation: Operation,
    operand: u32,
    test: u32,
    throw_true: u8,
    throw_false: u8,
    activity: u32,
    lcm: u32,
}
impl Monkey {
    fn check_item(&mut self, item: u32, part1: bool) -> (u8, u32) {
        let mut item = item as u64;
        self.activity += 1;

        match self.operation {
            Operation::Multiply => item *= self.operand as u64,
            Operation::Add => item += self.operand as u64,
            Operation::Power => item *= item,
        }
        if part1 { item /= 3; }
        else {
            while item >= self.lcm as u64 {
                item -= self.lcm as u64;
            }
        }
        if item as u32 % self.test == 0 { (self.throw_true, item as u32) }
        else { (self.throw_false, item as u32) }
    }
}

pub fn monkey_in_the_middle() {
    let test = false;

    let mut monkeys1: Vec<Monkey> = Vec::new();
    if test {
        monkeys1.push(Monkey {
            items: vec![79, 98],
            operation: Operation::Multiply,
            operand: 19,
            test: 23,
            throw_true: 2,
            throw_false: 3,
            activity: 0,
            lcm: 96577,
        });
        monkeys1.push(Monkey {
            items: vec![54, 65, 75, 74],
            operation: Operation::Add,
            operand: 6,
            test: 19,
            throw_true: 2,
            throw_false: 0,
            activity: 0,
            lcm: 96577,
        });
        monkeys1.push(Monkey {
            items: vec![79, 60, 97],
            operation: Operation::Power,
            operand: 0,
            test: 13,
            throw_true: 1,
            throw_false: 3,
            activity: 0,
            lcm: 96577,
        });
        monkeys1.push(Monkey {
            items: vec![74],
            operation: Operation::Add,
            operand: 3,
            test: 17,
            throw_true: 0,
            throw_false: 1,
            activity: 0,
            lcm: 96577,
        });
    } else {
        monkeys1.push(Monkey {
            items: vec![54, 61, 97, 63, 74],
            operation: Operation::Multiply,
            operand: 7,
            test: 17,
            throw_true: 5,
            throw_false: 3,
            activity: 0,
            lcm: 9699690,
        });
        monkeys1.push(Monkey {
            items: vec![61, 70, 97, 64, 99, 83, 52, 87],
            operation: Operation::Add,
            operand: 8,
            test: 2,
            throw_true: 7,
            throw_false: 6,
            activity: 0,
            lcm: 9699690,
        });
        monkeys1.push(Monkey {
            items: vec![60, 67, 80, 65],
            operation: Operation::Multiply,
            operand: 13,
            test: 5,
            throw_true: 1,
            throw_false: 6,
            activity: 0,
            lcm: 9699690,
        });
        monkeys1.push(Monkey {
            items: vec![61, 70, 76, 69, 82, 56],
            operation: Operation::Add,
            operand: 7,
            test: 3,
            throw_true: 5,
            throw_false: 2,
            activity: 0,
            lcm: 9699690,
        });
        monkeys1.push(Monkey {
            items: vec![79, 98],
            operation: Operation::Add,
            operand: 2,
            test: 7,
            throw_true: 0,
            throw_false: 3,
            activity: 0,
            lcm: 9699690,
        });
        monkeys1.push(Monkey {
            items: vec![72, 79, 55],
            operation: Operation::Add,
            operand: 1,
            test: 13,
            throw_true: 2,
            throw_false: 1,
            activity: 0,
            lcm: 9699690,
        });
        monkeys1.push(Monkey {
            items: vec![63],
            operation: Operation::Add,
            operand: 4,
            test: 19,
            throw_true: 7,
            throw_false: 4,
            activity: 0,
            lcm: 9699690,
        });
        monkeys1.push(Monkey {
            items: vec![72, 51, 93, 63, 80, 86, 81],
            operation: Operation::Power,
            operand: 0,
            test: 11,
            throw_true: 0,
            throw_false: 4,
            activity: 0,
            lcm: 9699690,
        });
    }
    let mut monkeys2 = monkeys1.clone();

    for _ in 0..20 {
        for monkey in 0..monkeys1.len() {
            for item in 0..monkeys1[monkey].items.len() {
                let item = monkeys1[monkey].items[item];
                let res = monkeys1[monkey].check_item(item, true);
                monkeys1[res.0 as usize].items.push(res.1);
            }
            monkeys1[monkey].items.clear();
        }
    }

    let business1 = get_business(&monkeys1);
    println!("Day 11 part 1: {}", business1);

    for round in 0..10000 {
        if round % 1000 == 0 { println!("Finished round {}", round); }
        for monkey in 0..monkeys2.len() {
            for item in 0..monkeys2[monkey].items.len() {
                let item = monkeys2[monkey].items[item];
                let res = monkeys2[monkey].check_item(item, false);
                monkeys2[res.0 as usize].items.push(res.1);
            }
            monkeys2[monkey].items.clear();
        }
    }

    let business2 = get_business(&monkeys2);
    println!("Day 11 part 2: {}", business2);
}

fn get_business(monkeys: &Vec<Monkey>) -> u128 {
    let mut largest = 0;
    let mut second = 0;
    for monkey in monkeys {
        if monkey.activity > largest {
            second = largest;
            largest = monkey.activity;
        } else if monkey.activity > second {
            second = monkey.activity;
        }
    }
    largest as u128 * second as u128
}
