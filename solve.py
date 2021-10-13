def solve(*, expected: int):
    def inner(func) -> None:
        actual = func()

        try:
            assert actual == expected
        except AssertionError:
            raise TestFailedError(actual, expected)

    return inner


class TestFailedError(Exception):
    def __init__(self, actual: int, expected: int) -> None:
        self.actual = actual
        self.expected = expected
