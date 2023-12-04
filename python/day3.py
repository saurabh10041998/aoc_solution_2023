from pathlib import PosixPath
from typing import List, Generator, Tuple, NamedTuple
import re

NUM = re.compile(r"\d+")


class Num(NamedTuple):
    n: int
    y: int
    x: tuple[int, int]

    def adjacent(self, x: int, y: int) -> bool:
        return self.x[0] - 1 <= x < self.x[1] + 1 and self.y - 1 <= y <= self.y + 1


def get_input(filename: str) -> List[str]:
    input_file_path = PosixPath(PosixPath.cwd(), filename)
    lines = []
    with open(input_file_path, "r") as f:
        lines = f.readlines()
    return [line.strip() for line in lines]


def adjacent_8(x: int, y: int) -> Generator[Tuple[int, int], None, None]:
    for dx in [-1, 0, 1]:
        for dy in [-1, 0, 1]:
            if dx == dy == 0:
                continue
            yield x + dx, y + dy


def p1(lines: List[str]) -> int:
    coords = {}
    total = 0
    for y, line in enumerate(lines):
        for x, c in enumerate(line):
            if c != "." and not c.isdigit():
                coords[(x, y)] = c

    for y, line in enumerate(lines):
        for match in NUM.finditer(line):
            matched = False
            for i in range(*match.span()):
                for cand_x, cand_y in adjacent_8(i, y):
                    if (cand_x, cand_y) in coords:
                        break
                else:
                    continue
                matched = True
            if matched:
                total += int(match[0])
    return total


def p2(lines: List[str]) -> int:
    total = 0
    stars = set()
    for y, line in enumerate(lines):
        for x, c in enumerate(line):
            if c == "*":
                stars.add((x, y))
    nums = []
    for y, line in enumerate(lines):
        for match in NUM.finditer(line):
            nums.append(Num(int(match[0]), y, match.span()))

    for x, y in stars:
        adj = [num for num in nums if num.adjacent(x, y)]
        if len(adj) == 2:
            total += adj[0].n * adj[1].n
    return total


def main() -> int:
    lines = get_input("input.txt")
    print("[+] Part 1 ans =>", p1(lines))
    print("[+] Part 2 ans =>", p2(lines))


if __name__ == "__main__":
    exit(main())
