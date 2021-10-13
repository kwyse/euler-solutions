from importlib import import_module
from multiprocessing import Pool
from pathlib import Path

from colorama import Fore

from solve import TestFailedError


def run_all():
    print("Project Euler Solutions")
    euler_paths = Path("./project_euler/").glob(r"p*.py")
    with Pool() as pool:
        pool.map(_run, (path.stem for path in euler_paths))


def _run(module):
    try:
        import_module(f"project_euler.{module}")
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
