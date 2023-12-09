#! /usr/bin/env python3
from pathlib import PosixPath
from typing import List, NamedTuple
from itertools import cycle
from math import lcm
import argparse
import re

BRANCH = re.compile("(\w\w\w) = \((\w\w\w), (\w\w\w)\)")


def get_input(filename: str) -> List[str]:
    input_file_path = PosixPath(PosixPath.cwd(), filename)
    lines = []
    with open(input_file_path, "r") as f:
        lines = f.readlines()
    return [line.strip() for line in lines]


class Branch(NamedTuple):
    left: str
    right: str


def part1(lines: List[str]) -> int:
    cmd = None
    navigator_map = {}
    for line in lines:
        if cmd is None:
            cmd = line
        elif line:
            matches = BRANCH.search(line)
            src, left, right = matches.group(1), matches.group(2), matches.group(3)
            navigator_map[src] = Branch(left, right)
    iterations = 0
    pool_of_instructions = cycle(cmd)
    current = "AAA"
    while current != "ZZZ":
        next_instruction = next(pool_of_instructions)
        if next_instruction == "L":
            current = navigator_map[current].left
        else:
            current = navigator_map[current].right
        iterations += 1
    return iterations


def part2(lines: List[str]) -> int:
    cmd = None
    navigator_map = {}
    sources = []
    for line in lines:
        if cmd is None:
            cmd = line
        elif line:
            matches = BRANCH.search(line)
            src, left, right = matches.group(1), matches.group(2), matches.group(3)
            if src[-1] == "A":
                sources.append(src)
            navigator_map[src] = Branch(left, right)
    iterations = []
    for source in sources:
        current = source
        iteration = 0
        pool = cycle(cmd)
        while current[-1] != "Z":
            next_instruction = next(pool)
            if next_instruction == "L":
                current = navigator_map[current].left
            else:
                current = navigator_map[current].right
            iteration += 1
        iterations.append(iteration)
    return lcm(*iterations)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("-i", "--input", required=True, help="Input file name")
    args = parser.parse_args()
    opts = vars(args)
    lines = get_input(opts["input"])
    print("[+] Part 1 ans =>", part1(lines))
    print("[+] Part 2 ans =>", part2(lines))


if __name__ == "__main__":
    exit(main())
