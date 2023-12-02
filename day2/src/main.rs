use regex::Regex;

use std::collections::HashMap;

fn part1(input: &str) -> i32 {
    let game_regex = Regex::new(r"Game (\d+):").unwrap();
    let cube_regex = Regex::new(r"(\d+) (red|green|blue)").unwrap();
    let max_cube_map = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    input
        .lines()
        .map(|s| {
            let game_cap = game_regex.captures(s).unwrap();
            let cube_caps = cube_regex.captures_iter(s);
            (game_cap, cube_caps)
        })
        .fold(0, |acc, (game_cap, cube_caps)| {
            for c in cube_caps {
                let (_, [n, color]) = c.extract();
                if max_cube_map[color] < n.parse().unwrap() {
                    return acc;
                }
            }
            let (_, [id]) = game_cap.extract();
            acc + id.parse::<i32>().unwrap()
        })
}

fn part2(input: &str) -> i32 {
    // Could lift this regex to argument or static, since it's the same one from part 1,
    // but on this scale there's no point
    let regex = Regex::new(r"(\d+) (red|green|blue)").unwrap();
    input
        .lines()
        .map(|s| regex.captures_iter(s))
        .fold(0, |acc, c| {
            let mut map = HashMap::new();
            for (_, [n, color]) in c.map(|c| c.extract()) {
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
    let input = include_str!("input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
