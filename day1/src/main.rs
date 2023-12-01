use regex::Regex;

fn part1(input: &str) -> i32 {
    let regex1 = Regex::new(r"(?m)^\D*(\d)\w*?(\d)\D*$").unwrap();
    let regex2 = Regex::new(r"(?m)^\D*(\d)\D*$").unwrap();
    let sum =
        regex1
            .captures_iter(input)
            .map(|m| m.extract())
            .fold(0, |acc, (_, [first, last])| {
                acc + first.parse::<i32>().unwrap() * 10 + last.parse::<i32>().unwrap()
            });
    sum + regex2
        .captures_iter(input)
        .map(|m| m.extract())
        .fold(0, |acc, (_, [only])| {
            let only: i32 = only.parse().unwrap();
            acc + only * 10 + only
        })
}

fn parse_digit(digit: &str) -> i32 {
    match digit {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => digit.parse().unwrap(),
    }
}

fn part2(input: &str) -> i32 {
    let regex1 =
        Regex::new(r"(?m)^\D*?(\d|one|two|three|four|five|six|seven|eight|nine)\w*(\d|one|two|three|four|five|six|seven|eight|nine)\D*$").unwrap();
    let regex2 = Regex::new(r"(?m)^\D*(\d)\D*$").unwrap();
    let sum = regex1
        .captures_iter(input)
        .map(|c| c.extract())
        .fold(0, |acc, (_, [first, last])| {
            acc + parse_digit(first) * 10 + parse_digit(last)
        });
    regex2
        .captures_iter(input)
        .map(|c| c.extract())
        .filter(|(full, _)| !regex1.is_match(full))
        .fold(sum, |acc, (_, [only])| {
            let only = parse_digit(only);
            acc + only * 10 + only
        })
}

fn main() {
    let input = include_str!("input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
