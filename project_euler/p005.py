from itertools import count

from . import solve


@solve(expected=232_792_560)
def p005():
    for i in count(20, 20):
        found = False

        for j in range(19, 2, -1):
            if i % j != 0:
                break

            if j == 3:
                found = True

        if found:
            return i
