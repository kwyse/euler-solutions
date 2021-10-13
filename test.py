from importlib import import_module
from multiprocessing import Pool
from pathlib import Path
from sys import argv

from colorama import Fore

from solve import TestFailedError


def __run_all(argv):
    if len(argv) > 1:
        _, arg, *_ = argv
        package, *module = arg.split(".")

        if module:
            modules = [arg]
        else:
            modules = __build_module_paths(package)
    else:
        modules = __build_module_paths("advent_of_code")
        modules.extend(__build_module_paths("hacker_rank"))
        modules.extend(__build_module_paths("project_euler"))

    with Pool() as pool:
        pool.map(__run, modules)


def __build_module_paths(package):
    paths = Path(f"./{package}").glob(r"*[!_].py")
    return [f"{package}.{path.stem}" for path in paths]


def __run(module):
    try:
        import_module(module)
        print(Fore.GREEN + f"- Solution {module} passed ✓")
    except TestFailedError as e:
        __error(f"! Solution {module} failed; expected: {e.expected}, got: {e.actual}")
    except Exception as e:
        __error(f"! Solution {module} failed to compile with error:", e)


def __error(msg, e=None):
    if e:
        print(Fore.RED + msg, e)
    else:
        print(Fore.RED + msg)


if __name__ == "__main__":
    __run_all(argv)
