#! /usr/bin/env python3
from pathlib import PosixPath
from typing import List


def get_input(filename: str) -> List[str]:
    input_file_path = PosixPath(PosixPath.cwd(), filename)
    lines = []
    with open(input_file_path, "r") as f:
        lines = f.readlines()
    return [line.strip() for line in lines]


def part1(lines: List[str]) -> int:
    total = 0
    for line in lines:
        _, cards = line.split(":")
        wincards, cardset = cards.split(" | ")
        wincards, cardset = wincards.strip(), cardset.strip()
        wincards = wincards.split(" ")
        winset = set()
        for w in wincards:
            if w:
                winset.add(int(w))
        lst = []
        for c in cardset.split(" "):
            if c:
                if int(c) in winset:
                    lst.append(c)
        total += int(pow(2, len(lst) - 1))
    return total


def part2(lines: List[str]) -> int:
    freq = {i: 1 for i in range(len(lines))}
    for i, line in enumerate(lines):
        _, cards = line.split(":")
        wincards, cardset = cards.split(" | ")
        wincards, cardset = wincards.strip(), cardset.strip()
        wincards = wincards.split(" ")
        winset = set()
        for w in wincards:
            if w:
                winset.add(int(w))
        lst = []
        for c in cardset.split(" "):
            if c:
                if int(c) in winset:
                    lst.append(c)
        for _ in range(freq[i]):
            for n in range(i + 1, i + 1 + len(lst)):
                freq[n] += 1
    return sum(freq.values())


def main() -> int:
    # lines = get_input("ex.txt")
    lines = get_input("input.txt")
    print("[+] Part 1 ans", part1(lines))
    print("[+] Part 2 ans", part2(lines))


if __name__ == "__main__":
    exit(main())
