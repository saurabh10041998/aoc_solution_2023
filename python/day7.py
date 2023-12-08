#! /usr/bin/env python3
from enum import IntEnum
from pathlib import PosixPath
from typing import List, Self


def get_input(filename: str) -> List[str]:
    input_file_path = PosixPath(PosixPath.cwd(), filename)
    lines = []
    with open(input_file_path, "r") as f:
        lines = f.readlines()
    return [line.strip() for line in lines]


class HandType(IntEnum):
    HIGHCARD = 1
    ONEPAIR = 2
    TWOPAIR = 3
    THREEOFKIND = 4
    FULLHOUSE = 5
    FOUROFKIND = 6
    FIVEOFKIND = 7


class Hand:
    def __init__(self, hand: str, bet: int, part2: bool = False) -> None:
        self.hand = hand
        self.bet = bet
        self.card_map = {"T": "A", "J": "B", "Q": "C", "K": "D", "A": "E"}
        if part2:
            self.card_map["J"] = "0"
        self.hand_type = self.get_hand_type(part2)

    def __repr__(self):
        return f"Hand: {self.hand} -> {self.hand_type.name} -> {self.bet}"

    def __eq__(self, other: Self) -> bool:
        return self.hand == other.hand

    def __gt__(self, other: Self) -> bool:
        if self.hand_type == other.hand_type:
            self_hand_map = [self.card_map.get(c, c) for c in self.hand]
            other_hand_map = [self.card_map.get(c, c) for c in other.hand]
            return self_hand_map > other_hand_map
        return self.hand_type > other.hand_type

    def get_hand_type(self, part2: bool) -> HandType:
        if not part2:
            return self.get_hand_type_inner(self.hand)
        return max(self.get_hand_type_inner(h) for h in self.replace_wild(self.hand))

    @staticmethod
    def replace_wild(hand: str) -> List[str]:
        if hand == "":
            return [""]
        return [
            a + b
            for a in ("AKQT98765432" if hand[0] == "J" else hand[0])
            for b in Hand.replace_wild(hand[1:])
        ]

    @staticmethod
    def get_hand_type_inner(hand: str) -> HandType:
        counts = [hand.count(c) for c in hand]
        if 5 in counts:
            return HandType.FIVEOFKIND
        if 4 in counts:
            return HandType.FOUROFKIND
        if 3 in counts and 2 in counts:
            return HandType.FULLHOUSE
        if 3 in counts:
            return HandType.THREEOFKIND
        if 2 in counts and counts.count(2) == 4:
            return HandType.TWOPAIR
        if 2 in counts:
            return HandType.ONEPAIR
        return HandType.HIGHCARD


def main() -> int:
    # lines = get_input("ex.txt")
    lines = get_input("input.txt")
    hands = []
    for line in lines:
        hand, bet = line.split(" ")
        hands.append(Hand(hand, int(bet)))
    part1 = 0
    for i, hand in enumerate(sorted(hands), 1):
        part1 += hand.bet * i
    hands = []
    for line in lines:
        hand, bet = line.split(" ")
        hands.append(Hand(hand, int(bet), part2=True))
    part2 = 0
    for i, hand in enumerate(sorted(hands), 1):
        part2 += hand.bet * i
    print("Part 1 answer =>", part1)
    print("Part 2 answer =>", part2)


if __name__ == "__main__":
    exit(main())
