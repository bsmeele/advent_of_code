use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn pulse_propagation() {
    let test = false;
    let filename = if test { "src/y2023/day20/test2" } else { "src/y2023/day20/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut network: HashMap<String, Module> = HashMap::new();
    let num_presses = 1000;
    let sand_machine = "rx";

    for line in reader.lines().map(|x| x.unwrap()) {
        let split = line.split('-').collect::<Vec<&str>>();
        let (module_type, name) = match line.chars().collect::<Vec<char>>().first().unwrap() {
            'b' => (ModuleType::Broadcast, String::from("broadcast")),
            '%' => (ModuleType::FlipFlop(FlipFlopStruct::new()), split[0].replace('%', "").replace(' ', "")),
            '&' => (ModuleType::Conjunction(ConjunctionStruct::new()), split[0].replace('&', "").replace(' ', "")),
            _ => panic!("Unreachable"),
        };
        let mut out: Vec<String> = Vec::new();
        for o in split[1].replace('>', "").split(',') { out.push(o.replace(' ', "")); }
        network.insert(name, Module{module_type, out});
    }

    let mut to_update: Vec<(String, Vec<String>)> = Vec::new();
    let mut out_list: Vec<String> = Vec::new();
    for (name, module) in &network {
        if let ModuleType::Conjunction(_) = &module.module_type {
            for (n, m) in &network {
                if m.out.contains(name) {
                    out_list.push(n.clone());
                }
            }
            to_update.push((name.clone(), out_list.clone()));
            out_list.clear();
        }
    }
    for (name, input) in to_update {
        if let ModuleType::Conjunction(ref mut c) = &mut network.get_mut(&name).unwrap().module_type { c.update_in(input); }
    }

    let backup_network = network.clone();

    let mut low_pulses = 0;
    let mut high_pulses = 0;
    let mut pulse: VecDeque<(String, bool, String)> = VecDeque::new();

    for _ in 0..num_presses {
        pulse.push_back((String::from("broadcast"), false, String::from("button")));
        while let Some((to, p, from)) = pulse.pop_front() {
            if p { high_pulses += 1; } else { low_pulses += 1; }
            if let Some(out) = network.get_mut(&to) {
                if let Some(o) = out.pulse(from, p) {
                    for m in o.0 {
                        pulse.push_back((m, o.1, to.clone()));
                    }
                }
            }
        }
    }

    println!("Year 2023 day 20 part 1: {}", low_pulses * high_pulses);

    if test { return; }

    // After inspection by flow chart:
    //   rx receives input from one conjunction called vf
    //   vf receives input from four conjunctions called pm, mk, pk, and hf
    //   pm outputs high with a period of 3881 presses
    //   mk outputs high with a period of 3889 presses
    //   hf outputs high with a period of 4013 presses
    //   pk outputs high with a period of 4021 presses
    // This results in rx receiving a low after lcm(3881, 3889, 4013, 4021) = 243_548_140_870_057 presses

    network = backup_network;
    let mut targets: HashSet<String> = HashSet::new();
    'find_target: for (_, m) in &network {
        for o in &m.out {
            if o == sand_machine {
                if let ModuleType::Conjunction(c) = &m.module_type {
                    for (t, _) in &c.memory {
                        targets.insert(t.clone());
                    }
                } ;
                break 'find_target;
            }
        }
    }

    let mut periods: HashMap<String, usize> = HashMap::new();
    let mut button_presses = 0;
    'press_loop: loop {
        button_presses += 1;
        pulse.push_back((String::from("broadcast"), false, String::from("button")));
        while let Some((to, p, from)) = pulse.pop_front() {
            if let Some(out) = network.get_mut(&to) {
                if let Some(o) = out.pulse(from, p) {
                    for m in o.0 {
                        if m == "rx" && !o.1 { break 'press_loop; }
                        if o.1 && targets.contains(&to) && !periods.contains_key(&to) {
                            periods.insert(to.clone(), button_presses);
                            if periods.len() == targets.len() { break 'press_loop; }
                        }
                        pulse.push_back((m, o.1, to.clone()));
                    }
                }
            }
        }
    }

    button_presses = 1;
    for e in periods.values() {
        button_presses = crate::y2023::day8::lcm(button_presses, *e);
    }

    println!("Year 2023 day 20 part 2: {}", button_presses);
}

#[derive(Debug, Clone)]
struct Module {
    module_type: ModuleType,
    out: Vec<String>,
}
impl Module {
    fn pulse(&mut self, from: String, pulse: bool) -> Option<(Vec<String>, bool)> {
        match &mut self.module_type {
            ModuleType::Broadcast => {
                if !pulse { Some((self.out.clone(), false)) }
                else { None }
            },
            ModuleType::FlipFlop(f) => {
                if let Some(p) = f.pulse(pulse) { Some((self.out.clone(), p)) }
                else { None }
            },
            ModuleType::Conjunction(c) =>  Some((self.out.clone(), c.pulse(from, pulse)))
        }
    }
}

#[derive(Debug, Clone)]
enum ModuleType {
    Broadcast,
    FlipFlop(FlipFlopStruct),
    Conjunction(ConjunctionStruct),
}

#[derive(Debug, Copy, Clone)]
struct FlipFlopStruct {
    memory: bool,
}
impl FlipFlopStruct {
    fn new() -> Self {
        Self{memory: false}
    }
    fn pulse(&mut self, pulse: bool) -> Option<bool> {
        if pulse { None }
        else {
            self.memory = !self.memory;
            Some(self.memory)
        }
    }
}

#[derive(Debug, Clone)]
struct ConjunctionStruct {
    memory: HashMap<String, bool>
}
impl ConjunctionStruct {
    fn new() -> Self {
        Self{memory: HashMap::new()}
    }
    fn update_in(&mut self, input: Vec<String>) {
        for i in input {
            self.memory.insert(i, false);
        }
    }
    fn pulse(&mut self, from: String, pulse: bool) -> bool {
        self.memory.insert(from, pulse);

        for (_, p) in &self.memory {
            if !p { return true; }
        }

        false
    }
}
