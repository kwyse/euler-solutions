from hashlib import md5
from itertools import count

from . import solve


@solve(expected=254_575)
def part1():
    secret = "bgvyzdsv"

    for n in count(1):
        md5_hash = md5((secret + str(n)).encode())
        if md5_hash.hexdigest().startswith("00000"):
            return n


@solve(expected=1_038_736)
def part2():
    secret = "bgvyzdsv"

    for n in count(254_575 + 1):
        md5_hash = md5((secret + str(n)).encode())
        if md5_hash.hexdigest().startswith("000000"):
            return n
