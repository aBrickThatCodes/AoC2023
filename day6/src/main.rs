fn part1(input: &str) -> i32 {
    let mut input = input.split('\n').filter(|s| !s.is_empty()).map(|s| {
        s.split_whitespace()
            .skip(1)
            .map(|s| s.parse::<i32>().unwrap())
    });
    let input = input.next().unwrap().zip(input.next().unwrap());
    let mut prod = 1;
    for (t, s) in input {
        let delta_sqrt = ((t.pow(2) - 4 * s) as f64).sqrt();
        let beg = ((t as f64 - delta_sqrt) / 2.0) as i32 + 1;
        let end = ((t as f64 + delta_sqrt) / 2.0) as i32;
        let n = (beg..=end).count() as i32;
        prod *= n;
    }
    prod
}

fn part2(input: &str) -> i32 {
    let mut input = input.split('\n').filter(|s| !s.is_empty()).map(|s| {
        s.chars()
            .filter(|c| *c != ' ')
            .skip_while(|c| *c != ':')
            .skip(1)
            .collect::<String>()
            .parse::<i64>()
            .unwrap()
    });
    let t = input.next().unwrap();
    let s = input.next().unwrap();
    let delta_sqrt = ((t.pow(2) - 4 * s) as f64).sqrt();
    let beg = ((t as f64 - delta_sqrt) / 2.0) as i32 + 1;
    let end = ((t as f64 + delta_sqrt) / 2.0) as i32;
    (beg..=end).count() as i32
}

fn main() {
    let input = include_str!("input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
