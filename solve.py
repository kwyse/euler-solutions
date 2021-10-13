from pathlib import Path


def solve(*, expected: int, resource=None, data_dir=None):
    def inner(func) -> None:
        if resource:
            with open(Path().resolve().joinpath(data_dir).joinpath(resource)) as f:
                data = f.read()
                actual = func(data)
        else:
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
