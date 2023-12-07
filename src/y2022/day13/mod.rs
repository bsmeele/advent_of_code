use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, PartialEq)]
enum Entry {
    Integer(u8),
    List(Vec<Entry>),
}
impl Entry {
    fn new(input: &mut VecDeque<char>) -> Vec<Self> {
        let mut packet: Vec<Self> = Vec::new();
        let mut num_string = String::new();

        while let Some(c) = input.pop_front() {
            match c {
                '[' => {
                    packet.push(Self::List(Self::new(input)));
                },
                ',' => {
                    if !num_string.is_empty() { packet.push(Self::Integer(num_string.parse::<u8>().unwrap())); }
                    num_string.clear();
                },
                ']' => {
                    if !num_string.is_empty() { packet.push(Self::Integer(num_string.parse::<u8>().unwrap())); }
                    return packet;
                },
                _ => if c.is_numeric() { num_string.push(c); },
            }
        }
        packet
    }
    fn check_ordered(first: &mut Vec<Self>, second: &mut Vec<Self>) -> Option<bool> {
        let ordered = match first.len().cmp(&second.len()) {
            Ordering::Greater => Some(false),
            Ordering::Less => Some(true),
            Ordering::Equal => None,
        };
        loop {
            // println!("{:?} {:?}", first, second);
            let f = if first.is_empty() { break; } else { first.remove(0) };
            let s = if second.is_empty() { break; } else { second.remove(0) };
            match (f, s) {
                (Entry::Integer(f), Entry::Integer(s)) => {
                    match f.cmp(&s) {
                        Ordering::Greater => return Some(false),
                        Ordering::Less => return Some(true),
                        _ => (),
                    }
                },
                (Entry::Integer(f), Entry::List(mut s)) => {
                    let mut f = vec![Entry::Integer(f)];
                    let ordered = Self::check_ordered(&mut f, s.as_mut());
                    if let Some(r) = ordered { return Some(r); }
                },
                (Entry::List(mut f), Entry::Integer(s)) => {
                    let mut s = vec![Entry::Integer(s)];
                    let ordered = Self::check_ordered(&mut f, s.as_mut());
                    if let Some(r) = ordered { return Some(r); }
                },
                (Entry::List(mut f), Entry::List(mut s)) => {
                    let ordered = Self::check_ordered(&mut f, s.as_mut());
                    if let Some(r) = ordered { return Some(r); }
                },
            }
        }
        ordered
    }
}

pub fn distress_signal() {
    let test = false;
    let filename = if test { "src/day13/test" } else { "src/day13/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut packet_list: Vec<Vec<Entry>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() { continue; }
        let mut line: VecDeque<char> = line.chars().collect();
        line.pop_front();
        packet_list.push(Entry::new(&mut line));
    }

    let mut total = 0;
    for i in 0..packet_list.len()/2 {
        let mut f = packet_list[i*2].clone();
        let mut s = packet_list[i*2 + 1].clone();
        if Entry::check_ordered(&mut f, &mut s).unwrap() { total += i + 1; }
    }

    let dp2 = vec![Entry::List(vec![Entry::Integer(2)])];
    let dp6 = vec![Entry::List(vec![Entry::Integer(6)])];

    packet_list.push(dp2.clone());
    packet_list.push(dp6.clone());

    for i in 0..packet_list.len()-1 {
        for j in i + 1..packet_list.len() {
            let mut f = packet_list[i].clone();
            let mut s = packet_list[j].clone();
            if Entry::check_ordered(&mut f, &mut s) == Some(false) {
                let tmp = packet_list[i].clone();
                packet_list[i] = packet_list[j].clone();
                packet_list[j] = tmp;
            }
        }
    }

    let mut dp2i = 0;
    let mut dp6i = 0;
    for (i, item) in packet_list.iter().enumerate() {
        let mut f = item.clone();
        let mut f2 = item.clone();
        let mut dp2 = dp2.clone();
        let mut dp6 = dp6.clone();
        if f == dp2 { dp2i = 0; }
        if Entry::check_ordered(&mut f, &mut dp2).is_none() { dp2i = i + 1; }
        else if Entry::check_ordered(&mut f2, &mut dp6).is_none() { dp6i = i + 1; }
    }

    println!("Day 13 part 1: {}", total);
    println!("Day 13 part 2: {}", dp2i * dp6i);
}
