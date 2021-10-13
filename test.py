from importlib import import_module
from multiprocessing import Pool
from pathlib import Path

from colorama import Fore

from solve import TestFailedError


def run_all():
    print(Fore.RESET + "Project Euler Solutions")
    paths = Path("./project_euler/").glob(r"p*.py")
    with Pool() as pool:
        pool.map(_run, (f"project_euler.{path.stem}" for path in paths))

    print(Fore.RESET + "Advent of Code Solutions")
    paths = Path("./advent_of_code/").glob(r"20*.py")
    with Pool() as pool:
        pool.map(_run, (f"advent_of_code.{path.stem}" for path in paths))


def _run(module):
    try:
        import_module(module)
        print(Fore.GREEN + f"- Solution {module} passed ✓")
    except TestFailedError as e:
        print(
            Fore.RED
            + f"! Solution {module} failed; expected: {e.expected:_}, got: {e.actual:_}"
        )
    except Exception as e:
        print(Fore.RED + f"! Solution {module} failed to compile with error:", e)


if __name__ == "__main__":
    run_all()
