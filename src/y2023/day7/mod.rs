use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn camel_cards() {
    let test = false;
    let filename = if test { "src/y2023/day7/test" } else { "src/y2023/day7/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut hands: Vec<(String, usize)> = Vec::new();
    let mut acc = 0;

    for line in reader.lines().map(|x| x.unwrap()) {
        let line = line.split(' ').collect::<Vec<&str>>();
        hands.push((String::from(line[0]), line[1].parse::<usize>().unwrap()));
    }

    hands.sort_by(|h1, h2| compare_hands(&*h1.0, &*h2.0, false));
    for i in 1..=hands.len() {
        acc += i * hands[i-1].1;
    }

    println!("Year 2023 day 7 part 1: {}", acc);

    hands.sort_by(|h1, h2| compare_hands(&*h1.0, &*h2.0, true));
    acc = 0;
    for i in 1..=hands.len() {
        acc += i * hands[i-1].1;
    }

    println!("Year 2023 day 7 part 2: {}", acc);
}

fn compare_hands(hand1: &str, hand2: &str, joker: bool) -> Ordering {
    let h1 = eval_hand(hand1, joker);
    let h2 = eval_hand(hand2, joker);

    match (h1, h2) {
        (_, _) if h1 == h2 => {
            let h1 = hand1.chars().collect::<Vec<char>>();
            let h2 = hand2.chars().collect::<Vec<char>>();
            for i in 0..h1.len() {
                match (h1[i], h2[i]) {
                    (_, _) if h1[i] == h2[i] => continue,
                    ('A', _) => return Ordering::Greater,
                    (_, 'A') => return Ordering::Less,
                    ('K', _) => return Ordering::Greater,
                    (_, 'K') => return Ordering::Less,
                    ('Q', _) => return Ordering::Greater,
                    (_, 'Q') => return Ordering::Less,
                    ('J', _) => return if joker { Ordering::Less } else { Ordering::Greater },
                    (_, 'J') => return if joker { Ordering::Greater } else { Ordering::Less },
                    ('T', _) => return Ordering::Greater,
                    (_, 'T') => return Ordering::Less,
                    _ => return if h1[i] > h2[i] { Ordering::Greater } else { Ordering::Less }
                }
            }
            Ordering::Equal
        },
        (HandType::FiveOfAKind, _) => Ordering::Greater,
        (_, HandType::FiveOfAKind) => Ordering::Less,
        (HandType::FourOfAKind, _) => Ordering::Greater,
        (_, HandType::FourOfAKind) => Ordering::Less,
        (HandType::FullHouse, _) => Ordering::Greater,
        (_, HandType::FullHouse) => Ordering::Less,
        (HandType::ThreeOfAKind, _) => Ordering::Greater,
        (_, HandType::ThreeOfAKind) => Ordering::Less,
        (HandType::TwoPair, _) => Ordering::Greater,
        (_, HandType::TwoPair) => Ordering::Less,
        (HandType::OnePair, _) => Ordering::Greater,
        (_, HandType::OnePair) => Ordering::Less,
        _ => panic!("Unreachable")
    }
}

fn eval_hand(hand: &str, joker: bool) -> HandType {
    let mut s = hand.chars().collect::<Vec<char>>();
    s.sort();

    let mut num_jokers = 0;
    for c in &s { if c == &'J' { num_jokers += 1; } }

    let h = if s[0] == s[1] && s[0] == s[2] && s[0] == s[3] && s[0] == s[4] { HandType::FiveOfAKind }
    else if s[0] == s[1] && s[0] == s[2] && s[0] == s[3] { HandType::FourOfAKind }
    else if s[1] == s[2] && s[1] == s[3] && s[1] == s[4] { HandType::FourOfAKind }
    else if s[0] == s[1] && s[0] == s[2] && s[3] == s[4] { HandType::FullHouse }
    else if s[0] == s[1] && s[2] == s[3] && s[2] == s[4] { HandType::FullHouse }
    else if s[0] == s[1] && s[0] == s[2] { HandType::ThreeOfAKind }
    else if s[1] == s[2] && s[1] == s[3] { HandType::ThreeOfAKind }
    else if s[2] == s[3] && s[2] == s[4] { HandType::ThreeOfAKind }
    else if s[0] == s[1] && s[2] == s[3] { HandType::TwoPair }
    else if s[0] == s[1] && s[3] == s[4] { HandType::TwoPair }
    else if s[1] == s[2] && s[3] == s[4] { HandType::TwoPair }
    else if s[0] == s[1] { HandType::OnePair }
    else if s[1] == s[2] { HandType::OnePair }
    else if s[2] == s[3] { HandType::OnePair }
    else if s[3] == s[4] { HandType::OnePair }
    else { HandType::HighCard };

    if joker {
        if h == HandType::FourOfAKind && num_jokers == 4 { HandType::FiveOfAKind }
        else if h == HandType::FourOfAKind && num_jokers == 1 { HandType::FiveOfAKind}
        else if h == HandType::FullHouse && num_jokers == 3 { HandType::FiveOfAKind}
        else if h == HandType::FullHouse && num_jokers == 2 { HandType::FiveOfAKind}
        else if h == HandType::ThreeOfAKind && num_jokers == 3 { HandType::FourOfAKind }
        else if h == HandType::ThreeOfAKind && num_jokers == 2 { HandType::FiveOfAKind }
        else if h == HandType::ThreeOfAKind && num_jokers == 1 { HandType::FourOfAKind }
        else if h == HandType::TwoPair && num_jokers == 2 { HandType::FourOfAKind }
        else if h == HandType::TwoPair && num_jokers == 1 { HandType::FullHouse }
        else if h == HandType::OnePair && num_jokers == 2 { HandType::ThreeOfAKind }
        else if h == HandType::OnePair && num_jokers == 1 { HandType::ThreeOfAKind }
        else if h == HandType::HighCard && num_jokers == 1 { HandType::OnePair }
        else { h }
    } else { h }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}