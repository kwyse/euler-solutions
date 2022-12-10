from . import solve


@solve(expected=25_164_150)
def p006():
    def sum_of_squares(xs):
        return sum(x ** 2 for x in xs)

    def square_of_sum(xs):
        return sum(xs) ** 2

    xs = range(1, 101)
    return square_of_sum(xs) - sum_of_squares(xs)
