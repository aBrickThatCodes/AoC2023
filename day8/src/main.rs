use num::integer::lcm;
use regex::Regex;

use std::{cell::Cell, collections::HashMap};

fn part1(input: &str) -> i32 {
    let regex = Regex::new(r"(\w{3})\s*=\s*\((\w{3}),\s*(\w{3})\)").unwrap();
    let mut input = input.lines();
    let instructions = input.next().unwrap().chars().cycle();
    input.next();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in input {
        let (_, [node, left, right]) = regex.captures(line).unwrap().extract();
        map.insert(node, (left, right));
    }
    let mut current = "AAA";
    let mut i = 0;
    for c in instructions {
        if c == 'L' {
            current = &map[current].0;
        } else {
            current = &map[current].1;
        }
        i += 1;
        if current == "ZZZ" {
            break;
        }
    }
    i
}

fn part2(input: &str) -> usize {
    let regex = Regex::new(r"(\w{3})\s*=\s*\((\w{3}),\s*(\w{3})\)").unwrap();

    let mut input = input.lines();

    let instructions = input.next().unwrap();

    input.next();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in input {
        let (_, [node, left, right]) = regex.captures(line).unwrap().extract();
        map.insert(node, (left, right));
    }

    let nodes = map
        .keys()
        .filter(|s| s.ends_with('A'))
        .map(Cell::new)
        .collect::<Vec<_>>();

    let mut steps = Vec::new();

    for node in nodes {
        for (i, c) in instructions.chars().cycle().enumerate() {
            if c == 'L' {
                node.replace(&map[node.get()].0);
            } else {
                node.replace(&map[node.get()].1);
            }

            if node.get().ends_with('Z') {
                steps.push(i + 1);
                break;
            }
        }
    }
    steps.into_iter().reduce(lcm).unwrap()
}

fn main() {
    let input = include_str!("input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
