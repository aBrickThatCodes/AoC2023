use regex::Regex;

use std::cmp::Ordering;
use std::collections::HashMap;

fn card_cmp(a: char, b: char, cards: &[char]) -> Ordering {
    cards
        .iter()
        .position(|&c| c == a)
        .unwrap()
        .cmp(&cards.iter().position(|&c| c == b).unwrap())
}

fn get_strength(hand: &str) -> i32 {
    let mut map: HashMap<char, i32> = HashMap::new();
    for c in hand.chars() {
        map.entry(c).and_modify(|n| *n += 1).or_insert(1);
    }
    match map.keys().len() {
        5 => 1,
        4 => 2,
        3 => {
            if *map.values().max().unwrap() == 2 {
                3
            } else {
                4
            }
        }
        2 => {
            if *map.values().max().unwrap() == 3 {
                5
            } else {
                6
            }
        }
        _ => 7,
    }
}

fn part1(input: &str) -> i32 {
    let cards = [
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];
    let regex = Regex::new(r"(\w{5})\s+(\d*)").unwrap();
    let mut input: Vec<_> = input
        .lines()
        .map(|s| {
            let (_, [hand, bid]) = regex.captures(s).unwrap().extract();
            (hand, bid.parse::<i32>().unwrap())
        })
        .collect();
    input.sort_by(|a, b| {
        let str_a = get_strength(a.0);
        let str_b = get_strength(b.0);
        match str_a.cmp(&str_b) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                for (a, b) in a.0.chars().zip(b.0.chars()) {
                    let cmp = card_cmp(a, b, &cards);
                    if cmp != Ordering::Equal {
                        return cmp;
                    }
                }
                Ordering::Equal
            }
            Ordering::Greater => Ordering::Greater,
        }
    });
    input
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, (_, bid))| acc + (i as i32 + 1) * bid)
}

fn get_strength_with_jokers(hand: &str) -> i32 {
    let mut map: HashMap<char, i32> = HashMap::new();
    let mut jokers = 0;
    for c in hand.chars() {
        if c != 'J' {
            map.entry(c).and_modify(|n| *n += 1).or_insert(1);
        } else {
            jokers += 1;
        }
    }
    for v in map.values_mut() {
        *v += jokers;
    }
    match map.keys().len() {
        5 => 1,
        4 => 2,
        3 => {
            if *map.values().max().unwrap() == 2 {
                3
            } else {
                4
            }
        }
        2 => {
            if *map.values().max().unwrap() == 3 {
                5
            } else {
                6
            }
        }
        _ => 7,
    }
}

fn part2(input: &str) -> i32 {
    let cards = [
        'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
    ];
    let regex = Regex::new(r"(\w{5})\s+(\d*)").unwrap();
    let mut input: Vec<_> = input
        .lines()
        .map(|s| {
            let (_, [hand, bid]) = regex.captures(s).unwrap().extract();
            (hand, bid.parse::<i32>().unwrap())
        })
        .collect();
    input.sort_by(|a, b| {
        let str_a = get_strength_with_jokers(a.0);
        let str_b = get_strength_with_jokers(b.0);
        match str_a.cmp(&str_b) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                for (a, b) in a.0.chars().zip(b.0.chars()) {
                    let cmp = card_cmp(a, b, &cards);
                    if cmp != Ordering::Equal {
                        return cmp;
                    }
                }
                Ordering::Equal
            }
            Ordering::Greater => Ordering::Greater,
        }
    });
    input
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, (_, bid))| acc + (i as i32 + 1) * bid)
}

fn main() {
    let input = include_str!("input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
