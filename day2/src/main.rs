use regex::Regex;

use std::collections::HashMap;

fn part1(input: &str) -> i32 {
    let regex = Regex::new(r"(\d+) (red|green|blue)").unwrap();
    let max_cube_map = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    input.lines().enumerate().fold(0, |acc, (i, s)| {
        for c in regex.captures_iter(s) {
            let (_, [n, color]) = c.extract();
            if max_cube_map[color] < n.parse().unwrap() {
                return acc;
            }
        }

        acc + i as i32 + 1
    })
}

fn part2(input: &str) -> i32 {
    // Could lift this regex to argument or static, since it's the same one from part 1,
    // but on this scale there's no point
    let regex = Regex::new(r"(\d+) (red|green|blue)").unwrap();
    input.lines().fold(0, |acc, s| {
        let mut map = HashMap::new();
        for (_, [n, color]) in regex.captures_iter(s).map(|c| c.extract()) {
            let n: i32 = n.parse().unwrap();
            map.entry(color)
                .and_modify(|v| {
                    if *v < n {
                        *v = n;
                    }
                })
                .or_insert(n);
        }
        acc + map.into_values().reduce(|acc, v| acc * v).unwrap()
    })
}

fn main() {
    let input = include_str!("input").trim();
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
