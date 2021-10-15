from . import solve


def is_palindrome(n):
    return str(n) == str(n)[::-1]


@solve(expected=906_609)
def p004():
    candidates = []
    for n in range(100, 1000):
        for m in range(100, 1000):
            candidates.append(n * m)

    return max(c for c in candidates if is_palindrome(c))
