import os.path
import re

map_regex = re.compile(r"(?<=\n)(\d+)\s+(\d+)\s+(\d+)")


def process_map(in_str: str) -> tuple[int, int, int]:
    dest, src, rng = [int(x) for x in in_str.split()]
    return src, rng, dest


def part1(in_str) -> int:
    parts = in_str.split("\n\n")
    seeds = [int(x) for x in re.findall(r"(\d+)", parts[0])]

    for part in parts[1:]:
        part_map: list[tuple[int, int, int]] = list()

        for line in part.splitlines()[1:]:
            part_map.append(process_map(line))

        for i, seed in enumerate(seeds):
            for src_beg, rng, dest_beg in part_map:
                temp = seed - src_beg
                if 0 <= temp < rng:
                    seeds[i] = dest_beg + temp
                    break

    return min(seeds)


if __name__ == "__main__":
    with open(os.path.join(os.path.dirname(__file__), "input"), "r") as f:
        in_str = f.read().strip()
    print("Part 1:", part1(in_str))
