#! /usr/bin/env python3

from pathlib import PosixPath
from typing import List, Tuple
from collections import defaultdict
from functools import reduce


configuration = defaultdict(int, {"red": 12, "green": 13, "blue": 14})


def get_input(filename: str) -> List[str]:
    example_file_path = PosixPath(PosixPath.cwd(), filename)
    lines = []
    with open(example_file_path, "r") as f:
        lines = f.readlines()
    return [line.strip() for line in lines]


# Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
def p1_and_p2(lines: List[str]) -> Tuple[int, int]:
    total = 0
    total2 = 0
    for line in lines:
        game_id_str, rest_str = line.split(": ")
        _, game_id = game_id_str.split(" ")
        game_id = int(game_id)
        is_valid = True
        p2_config = defaultdict(int)
        for rnd in rest_str.split("; "):
            colors = rnd.split(", ")
            for c in colors:
                val, color = c.split(" ")
                val = int(val)
                if configuration[color] < val:
                    is_valid = False
                p2_config[color] = max(p2_config[color], val)
        if is_valid:
            total += game_id
        p2_ans = reduce(lambda x, y: x * y, p2_config.values(), 1)
        total2 += p2_ans
    return total, total2


def main():
    # lines = get_input("ex.txt")
    lines = get_input("input.txt")
    p1, p2 = p1_and_p2(lines)
    print("part1: ", p1)
    print("part2: ", p2)


if __name__ == "__main__":
    exit(main())
