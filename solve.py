from pathlib import Path


def solve(*, expected=None, resource=None, data_dir=None, test_cases=None):
    def __read_resource(resource: str) -> str:
        with open(Path().resolve().joinpath(data_dir).joinpath(resource)) as f:
            return f.read()

    def inner(func) -> None:
        if test_cases:
            for test_case in test_cases:
                if data_resource := test_case.get("data_resource"):
                    data = __read_resource(data_resource)
                    actual = func(data)
                else:
                    actual = func(test_case["data"])

                __assert(actual, test_case["expected"])

        else:
            if resource:
                data = __read_resource(resource)
                actual = func(data)
            else:
                actual = func()

            __assert(actual, expected)

    return inner


def __assert(actual, expected):
    try:
        assert actual == expected
    except AssertionError:
        raise TestFailedError(actual, expected)


class TestFailedError(Exception):
    def __init__(self, actual: int, expected: int) -> None:
        self.actual = actual
        self.expected = expected
