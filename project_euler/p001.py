from math import prod

from . import solve


@solve(expected=233_168)
def p001() -> int:
    xs, n = (3, 5), 1_000

    def sum_to(n):
        return (n * (n + 1)) // 2

    def sum_multiples(x):
        return x * sum_to((n - 1) // x)

    multiples = sum(sum_multiples(x) for x in xs)
    product_multiples = sum_multiples(prod(xs))

    return multiples - product_multiples
