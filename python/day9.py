from pathlib import PosixPath
from typing import List
import argparse


def get_input(filename: str) -> List[str]:
    input_file_path = PosixPath(PosixPath.cwd(), filename)
    lines = []
    with open(input_file_path, "r") as f:
        lines = f.readlines()
    return [line.strip() for line in lines]


def seq_diff(seq: List[int]) -> List[int]:
    return [y - x for x, y in zip(seq, seq[1:])]


def get_next(seq: List[int]) -> int:
    res = []
    while any(y - x != 0 for x, y in zip(seq, seq[1:])):
        seq = seq_diff(seq)
        res.append(seq[-1])
    return sum(res)


def part1(lines: List[str]) -> int:
    ans = 0
    for line in lines:
        lst = list(map(int, line.split(" ")))
        ans += lst[-1] + get_next(lst)
    return ans


def part2(lines: List[str]) -> int:
    ans = 0
    for line in lines:
        lst = list(map(int, line.split(" ")))
        ans += lst[0] + get_next(lst[::-1])
    return ans


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("-i", "--input", required=True, help="Input file name")
    args = parser.parse_args()
    opts = vars(args)
    lines = get_input(opts["input"])
    print("[+] Part 1 ans =>", part1(lines))
    print("[+] Part 2 ans =>", part2(lines))


if __name__ == "__main__":
    exit(main())
