#! /usr/bin/env python3
from pathlib import PosixPath
from typing import List, Optional, Tuple
import re


def sample_input_reader(input: str) -> List[str]:
    return input.strip().split("\n")


def get_input(day: int) -> List[str]:
    input_file_location = PosixPath(PosixPath.home(), "input", f"day{day}", "input.txt")
    with open(input_file_location, "r") as f:
        lines = f.readlines()
    return lines


def p1(lines: List[str]) -> int:
    total = 0
    for line in lines:
        digits = re.sub(r"\D", "", line)
        total += int(digits[0]) * 10 + int(digits[-1])
    return total


def p2(lines: List[str]) -> int:
    total = 0
    mappings = {
        "one": "one1one",
        "two": "two2two",
        "three": "three3three",
        "four": "four4four",
        "five": "five5five",
        "six": "six6six",
        "seven": "seven7seven",
        "eight": "eight8eight",
        "nine": "nine9nine",
    }
    for line in lines:
        for s, i in mappings.items():
            line = line.replace(s, i)
        digits = re.sub(r"\D", "", line)
        total += int(digits[0]) * 10 + int(digits[-1])
    return total


def main():
    sample_input = f"""
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
    """
    sample_input2 = f"""
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
    """
    # lines = sample_input_reader(sample_input)
    # lines = sample_input_reader(sample_input2)
    lines = get_input(1)
    print("[+] Part 1 answer is: ", p1(lines))
    print("[+] Part 2 answer is: ", p2(lines))


if __name__ == "__main__":
    exit(main())
