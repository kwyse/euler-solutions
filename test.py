from importlib import import_module
from multiprocessing import Pool
from pathlib import Path
from sys import argv

from colorama import Fore

from solve import TestFailedError


def _run_all(argv):
    if len(argv) > 1:
        _, arg, *_ = argv
        package, *module = arg.split(".")

        if module:
            modules = [arg]
        else:
            modules = _build_module_paths(package, r"*[!_].py")
    else:
        modules = _build_module_paths("advent_of_code", r"20*.py")
        modules.extend(_build_module_paths("hacker_rank", r"*[!_].py"))
        modules.extend(_build_module_paths("project_euler", r"p*.py"))

    with Pool() as pool:
        pool.map(_run, modules)


def _build_module_paths(package, pattern):
    paths = Path(f"./{package}/").glob(pattern)
    return [f"{package}.{path.stem}" for path in paths]


def _run(module):
    try:
        import_module(module)
        print(Fore.GREEN + f"- Solution {module} passed ✓")
    except TestFailedError as e:
        error(f"! Solution {module} failed; expected: {e.expected}, got: {e.actual}", e)
    except Exception as e:
        error(f"! Solution {module} failed to compile with error:", e)


def error(msg, e):
    print(Fore.RED + msg, e)


if __name__ == "__main__":
    _run_all(argv)
