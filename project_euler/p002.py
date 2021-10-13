from . import solve


@solve(expected=4_613_732)
def p002():
    a, b = 1, 2
    fibs = [a, b]

    while b < 4_000_000:
        a, b = b, a + b
        fibs.append(b)

    return sum(n for n in fibs if n % 2 == 0)
