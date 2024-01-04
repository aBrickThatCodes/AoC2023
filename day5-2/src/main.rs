use regex::Regex;

fn part2(input: &str) -> i64 {
    let seed_regex = Regex::new(r"(\d+)\s+(\d+)").unwrap();
    let map_regex = Regex::new(r"(\d+)\s+(\d+)\s+(\d+)").unwrap();

    let mut parts = input.split("\n\n");
    let seeds = parts.next().unwrap();
    let seeds = seeds.split(':').nth(1).unwrap();
    let seeds: Vec<_> = seed_regex
        .captures_iter(seeds)
        .map(|c| {
            let (_, [source, range]) = c.extract();
            (
                source.parse::<i64>().unwrap(),
                range.parse::<i64>().unwrap(),
            )
        })
        .collect();

    let mapping = parts
        .map(|s| {
            let maps = s
                .split('\n')
                .skip(1)
                .flat_map(|s| {
                    map_regex.captures_iter(s).map(|c| {
                        let (_, [dest, src, range]) = c.extract();
                        let dest: i64 = dest.parse().unwrap();
                        let src: i64 = src.parse().unwrap();
                        let range: i64 = range.parse().unwrap();
                        (dest, src, range)
                    })
                })
                .collect::<Vec<_>>();
            Box::new(move |x: i64| {
                for (dest, src, range) in &maps {
                    let dist = x - dest;
                    if dist >= 0 && dist < *range {
                        return src + dist;
                    }
                }
                x
            }) as Box<dyn Fn(i64) -> i64>
        })
        .reduce(|acc, f| Box::new(move |x| acc(f(x))))
        .unwrap();

    let mut i = 0;
    loop {
        for (src, range) in seeds.clone() {
            let var = mapping(i) - src;
            if var >= 0 && var < range {
                return i;
            }
        }
        i += 1;
    }
}

fn main() {
    let input = include_str!("input").trim();
    println!("Part 2: {}", part2(input));
}
