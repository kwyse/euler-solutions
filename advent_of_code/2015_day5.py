from itertools import pairwise, tee

from . import solve


@solve(expected=258, resource="2015_day5.txt")
def part1(data):
    rejected_pairs = ["ab", "cd", "pq", "xy"]
    vowels = list("aeiou")

    def is_nice(s: str):
        num_vowels = 1 if s[0] in vowels else 0
        double_letter_found = False

        for a, b in pairwise(s):
            pair = a + b
            if pair in rejected_pairs:
                return False

            if b in vowels:
                num_vowels += 1

            if a == b:
                double_letter_found = True

        return num_vowels >= 3 and double_letter_found

    return len([s for s in data.splitlines() if is_nice(s)])


def triplewise(iterable):
    a, b, c = tee(iterable, 3)
    next(b, None)
    next(c, None)
    next(c, None)

    for a, (b, c) in zip(a, zip(b, c)):
        yield a, b, c


@solve(expected=53, resource="2015_day5.txt")
def part2(data):
    def is_nice(s):
        non_overlapping_pair = False
        repeating_split_letter = False

        pairs = {s[:2]: 1}
        for i, (l, m, r) in enumerate(triplewise(s)):
            pair = m + r
            if found := pairs.get(pair):
                if found != i + 1:
                    non_overlapping_pair = True
            else:
                pairs[pair] = i + 2

            if l == r:
                repeating_split_letter = True

            if non_overlapping_pair and repeating_split_letter:
                return True
        
        return False

    return len([s for s in data.splitlines() if is_nice(s)])
