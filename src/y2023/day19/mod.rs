use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn aplenty() {
    let test = false;
    let filename = if test { "src/y2023/day19/test" } else { "src/y2023/day19/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut workflows: HashMap<String, Vec<Rule>> = HashMap::new();
    let mut parts: Vec<Part> = Vec::new();
    let mut accepted: Vec<Part> = Vec::new();
    let mut rejected: Vec<Part> = Vec::new();
    let mut flag = false;

    for line in reader.lines().map(|x| x.unwrap()) {
        if line.is_empty() {
            flag = true;
            continue;
        }
        if flag {
            let line = line.replace('{', "").replace('}', "");
            let part = line.split(',').collect::<Vec<&str>>();
            let x = part[0].split('=').collect::<Vec<&str>>()[1].parse::<usize>().unwrap();
            let m = part[1].split('=').collect::<Vec<&str>>()[1].parse::<usize>().unwrap();
            let a = part[2].split('=').collect::<Vec<&str>>()[1].parse::<usize>().unwrap();
            let s = part[3].split('=').collect::<Vec<&str>>()[1].parse::<usize>().unwrap();
            parts.push(Part { x, m, s, a });
        } else {
            let line = line.split('{');
            let workflow = line.collect::<Vec<&str>>();
            let line = workflow[1].replace('}', "");
            let rule_strings = line.split(',').collect::<Vec<&str>>();
            let mut rules: Vec<Rule> = Vec::new();
            for rule in rule_strings {
                let split = rule.split(':').collect::<Vec<&str>>();
                if split.len() == 1 {
                    rules.push(Rule { cat: '-', comp: Comp::None, num: 0, dest: String::from(split[0]) });
                    continue;
                }

                let chars = rule.chars().collect::<Vec<char>>();
                let cat = chars[0];
                let comp = match chars[1] {
                    '>' => Comp::Greater,
                    '<' => Comp::Lesser,
                    _ => panic!("Unreachable")
                };
                let num = split[0].replace('<', "").replace('>', "").replace('x', "").replace('m', "").replace('a', "").replace('s', "").parse::<usize>().unwrap();
                let dest = String::from(split[1]);
                rules.push(Rule { cat, comp, num, dest })
            }
            workflows.insert(String::from(workflow[0]), rules);
        }
    }

    let mut workflow: String;
    for part in parts {
        workflow = String::from("in");
        loop {
            if workflow == "A" {
                accepted.push(part);
                break;
            }
            if workflow == "R" {
                rejected.push(part);
                break;
            }
            for rule in &workflows[&workflow] {
                if let Some(w) = rule.apply(&part) {
                    workflow = w;
                    break;
                }
            }
        }
    }

    let mut acc = 0;
    for part in accepted {
        acc += part.x + part.m + part.a + part.s;
    }

    println!("Year 2023 day 19 part 1: {}", acc);

    let acc = score(&workflows, PartRange { x: (1, 4000), m: (1, 4000), a: (1, 4000), s: (1, 4000) }, String::from("in"));
    println!("Year 2023 day 19 part 2: {}", acc);
}

fn score(workspaces: &HashMap<String, Vec<Rule>>, part: PartRange, workspace: String) -> usize {
    if workspace == "R" { return 0; }
    if workspace == "A" { return (part.x.1 - part.x.0 + 1) * (part.m.1 - part.m.0 + 1) * (part.a.1 - part.a.0 + 1) * (part.s.1 - part.s.0 + 1); }

    let mut part = part;
    let mut parts: Vec<(PartRange, String)> = Vec::new();
    for rule in &workspaces[&workspace] {
        match rule.apply_range(part) {
            (None, Some(p)) => part = p,
            (Some((p, w)), None) => {
                parts.push((p, w));
                break;
            },
            (Some((p1, w)), Some(p2)) => {
                parts.push((p1, w));
                part = p2;
            },
            _ => panic!("Unreachable"),
        }
    }

    let mut acc = 0;
    for (part, workspace) in parts {
        acc += score(workspaces, part, workspace);
    }

    acc
}

#[derive(Copy, Clone, Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}
impl Part {
    fn get_cat(&self, cat: char) -> usize {
        match cat {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!("Unreachable"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct PartRange {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}
impl PartRange {
    fn get_cat(&self, cat: char) -> (usize, usize) {
        match cat {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!("Unreachable"),
        }
    }
    fn update_cat(&self, cat: char, update: (usize, usize)) -> Self {
        let mut part = self.clone();
        match cat {
            'x' => part.x = update,
            'm' => part.m = update,
            'a' => part.a = update,
            's' => part.s = update,
            _ => panic!("Unreachable"),
        }
        part
    }
}

#[derive(Debug)]
struct Rule {
    cat: char,
    comp: Comp,
    num: usize,
    dest: String,
}
impl Rule {
    fn apply(&self, part: &Part) -> Option<String> {
        match self.comp {
            Comp::Greater => if part.get_cat(self.cat) > self.num { Some(self.dest.clone()) } else { None },
            Comp::Lesser => if part.get_cat(self.cat) < self.num { Some(self.dest.clone()) } else { None },
            Comp::None => Some(self.dest.clone()),
        }
    }
    fn apply_range(&self, part: PartRange) -> (Option<(PartRange, String)>, Option<PartRange>) {
        match self.comp {
            Comp::Greater => {
                let cat = part.get_cat(self.cat);
                if cat.1 <= self.num { (None, Some(part)) }
                else if cat.0 > self.num { (Some((part, self.dest.clone())), None) }
                else { (Some((part.update_cat(self.cat, (self.num+1, cat.1)), self.dest.clone())), Some(part.update_cat(self.cat, (cat.0, self.num)))) }
            },
            Comp::Lesser => {
                let cat = part.get_cat(self.cat);
                if cat.0 >= self.num { (None, Some(part)) }
                else if cat.1 < self.num { (Some((part, self.dest.clone())), None) }
                else { (Some((part.update_cat(self.cat, (cat.0, self.num-1)), self.dest.clone())), Some(part.update_cat(self.cat, (self.num, cat.1)))) }
            },
            Comp::None => (Some((part, self.dest.clone())), None),
        }
    }
}

#[derive(Debug)]
enum Comp {
    Greater,
    Lesser,
    None,
}