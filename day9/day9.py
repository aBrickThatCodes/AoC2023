import os
from itertools import count


def is_last_zeros(iter) -> bool:
    return sum(1 for x in filter(lambda x: x != 0, iter[-1])) != 0


def get_diffs(list_of_vals: list[list[int]]) -> list[list[int]]:
    while is_last_zeros(list_of_vals):
        last_list = list_of_vals[-1]
        list_of_vals.append([y - x for x, y in zip(last_list[:-1], last_list[1:])])
    return list_of_vals


def part1(lines: list[str]) -> int:
    extras_sum = 0
    for line in lines:
        values = [int(x) for x in line.split()]
        diffs = list()
        diffs.append(values)
        diffs = get_diffs(diffs)
        diffs.reverse()
        diffs[0].append(0)
        for i, l in enumerate(diffs[1:]):
            diffs[i + 1].append(diffs[i + 1][-1] + diffs[i][-1])
        extras_sum += diffs[-1][-1]
    return extras_sum


def part2(lines: list[str]) -> int:
    extras_sum = 0
    for line in lines:
        values = [int(x) for x in line.split()]
        diffs = list()
        diffs.append(values)
        diffs = get_diffs(diffs)
        diffs.reverse()
        diffs[0].append(0)
        for i, l in enumerate(diffs[1:]):
            diffs[i + 1].insert(0, diffs[i + 1][0] - diffs[i][0])
        extras_sum += diffs[-1][0]
    return extras_sum


if __name__ == "__main__":
    with open(os.path.join(os.path.dirname(__file__), "input"), "r") as f:
        lines = f.readlines()
    print("Part 1:", part1(lines))
    print("Part 2:", part2(lines))
