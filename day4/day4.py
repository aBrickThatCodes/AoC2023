import os.path
import re

game_regex = re.compile(r"Card\s+\d+:")
numbers_regex = re.compile(r"(\d+)(?!:)")


def get_win_from_card(line) -> int:
    split = line.split("|")
    winning_numbers = numbers_regex.findall(split[1])
    numbers = list(
        filter(
            lambda x: x in winning_numbers,
            numbers_regex.findall(game_regex.sub("", split[0])),
        )
    )
    return len(numbers)


def part1(lines) -> int:
    sum = 0
    for line in lines:
        wins = get_win_from_card(line)
        if wins > 0:
            sum += 2 ** (wins - 1)
    return sum


def part2(lines) -> int:
    n_copies = [1] * len(lines)

    for i, line in enumerate(lines):
        wins = get_win_from_card(line)
        for j in range(i + 1, i + 1 + wins):
            n_copies[j] += n_copies[i]

    return sum(n_copies)


if __name__ == "__main__":
    lines = open(os.path.join(os.path.dirname(__file__), "input"), "r").readlines()
    print("Part 1:", part1(lines))
    print("Part 2:", part2(lines))
