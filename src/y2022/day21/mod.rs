use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn monkey_math() {
    let test = false;
    let filename = if test { "src/y2022/day21/test" } else { "src/y2022/day21/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut monkeys: HashMap<String, Monkey> = HashMap::new();
    let me = "humn";

    for line in reader.lines().map(|x| x.unwrap()) {
        let line: Vec<&str> = line.split(' ').collect();
        let name = line[0].replace(':', "");
        if line.len() == 2 {
            monkeys.insert(name, Monkey::Num(line[1].parse::<isize>().unwrap()));
        } else if line.len() == 4 {
            monkeys.insert(name, Monkey::Operation((String::from(line[1]), Operation::from_string(line[2]), String::from(line[3]))));
        }
    }

    // let mut tmp = 0;
    // for (_, m) in &monkeys {
    //     if let Monkey::Operation(_) = m { tmp += 1; }
    // }
    // println!("{}", tmp);

    let mut to_add: Vec<(String, Monkey)> = Vec::new();
    loop {
        for (s, m) in &monkeys {
            if let Monkey::Operation(_) = m {
                if let Ok(num) = monkey_yell(s, &monkeys, me, true) {
                    to_add.push((s.clone(), Monkey::Num(num)));
                }
            }
        }
        if to_add.is_empty() { break; }
        for e in &to_add { monkeys.insert(e.0.clone(), e.1.clone()); }
        to_add.clear();
    }

    // let mut tmp = 0;
    // for (_, m) in &monkeys {
    //     if let Monkey::Operation(_) = m { tmp += 1; }
    // }
    // println!("{}", tmp);
    // for (s, m) in &monkeys {
    //     println!("{} {:?}", s, m);
    // }


    let (m1, m2) = if let Monkey::Operation((m1, _, m2)) = &monkeys["root"] { (m1.clone(), m2.clone()) } else { panic!() };

    let num = match (monkey_yell(&m1, &monkeys, me, true), monkey_yell(&m2, &monkeys, me, true)) {
        (Ok(_), Ok(_)) => panic!(),
        (Ok(n), Err(())) => reverse(&m2, &monkeys, me, n).unwrap(),
        (Err(()), Ok(n)) => reverse(&m1, &monkeys, me, n).unwrap(),
        (Err(()), Err(())) => {
            let mut num = 0;
            loop {
                monkeys.insert(String::from("humn"), Monkey::Num(num));
                if let (Ok(m1), Ok(m2)) = (monkey_yell(&m1, &monkeys, me, false), monkey_yell(&m2, &monkeys, me, false)) {
                    if m1 == m2 { break; }
                }
                num += 1;
            }
            num
        }
    };

    println!("Year 2022 day 21 part 1: {}", monkey_yell("root", &monkeys, me, false).unwrap());
    println!("Year 2022 day 21 part 2: {}", num);
}

#[derive(Debug, Clone)]
enum Monkey {
    Num(isize),
    Operation((String, Operation, String)),
}

#[derive(Debug, Clone)]
enum Operation {
    Plus,
    Minus,
    Mult,
    Div,
}
impl Operation {
    fn from_string(s: &str) -> Self {
        match s {
            "+" => Self::Plus,
            "-" => Self::Minus,
            "*" => Self::Mult,
            "/" => Self::Div,
            _ => panic!("{s} is not a recognized operation")
        }
    }
}

fn monkey_yell(s: &str, monkeys: &HashMap<String, Monkey>, me: &str, part2: bool) -> Result<isize, ()> {
    if part2 && s == me { return Err(()); }
    match &monkeys[s] {
        Monkey::Num(n) => Ok(*n),
        Monkey::Operation((s1, op, s2)) => {
            match (monkey_yell(s1, monkeys, me, part2), monkey_yell(s2, monkeys, me, part2)) {
                (Ok(m1),Ok(m2)) => match op {
                    Operation::Plus => Ok(m1 + m2),
                    Operation::Minus => Ok(m1 - m2),
                    Operation::Mult => Ok(m1 * m2),
                    Operation::Div => Ok(m1 / m2),
                },
                _ => Err(()),
            }
        }
    }
}

fn reverse(s: &str, monkeys: &HashMap<String, Monkey>, me: &str, goal: isize) -> Result<isize, ()> {
    if s == me { return Ok(goal); }

    match &monkeys[s] {
        Monkey::Num(_) => Err(()),
        Monkey::Operation((s1, op, s2)) => {
            let (s, n, first) = match (monkey_yell(s1, monkeys, me, true), monkey_yell(s2, monkeys, me, true)) {
                (Err(()), Ok(n)) => (s1, n, true),
                (Ok(n), Err(())) => (s2, n, false),
                _ => return Err(()),
            };
            match op {
                Operation::Plus => reverse(s, monkeys, me, goal - n),
                Operation::Minus => {
                    if first { reverse(s, monkeys, me, goal + n) }
                    else { reverse(s, monkeys, me, n - goal) }
                },
                Operation::Mult => reverse(s, monkeys, me, goal / n),
                Operation::Div => {
                    if first { reverse(s, monkeys, me, goal * n) }
                    else { reverse(s, monkeys, me, n / goal) }
                },
            }
        },
    }
}

#[allow(dead_code)]
fn is_reversible(s: &str, monkeys: &HashMap<String, Monkey>, me: &str) -> bool {
    if s == me { return true; }

    match &monkeys[s] {
        Monkey::Num(_) => false,
        Monkey::Operation((s1, _, s2)) => {
            match (monkey_yell(s1, monkeys, me, true), monkey_yell(s2, monkeys, me, true)) {
                (Ok(_), Err(())) => is_reversible(s2, monkeys, me),
                (Err(()), Ok(_)) => is_reversible(s1, monkeys, me),
                _ => {
                    println!("{} {:?}", s, monkeys[s]);
                    false
                },
            }
        }
    }
}