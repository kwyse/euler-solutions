from math import floor, sqrt

from . import solve


def is_prime(n):
    for i in range(2, floor(sqrt(n)) + 1):
        if n % i == 0:
            return False

    return True


@solve(expected=6_857)
def p003():
    limit = 600_851_475_143

    factors = []
    factor = 2
    while limit > 1:
        if limit % factor == 0:
            factors.append(factor)
            limit /= factor
        else:
            factor += 1

    return max(f for f in factors if is_prime(f))
