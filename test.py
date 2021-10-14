from enum import Enum
from importlib import import_module
from multiprocessing import Pool
from pathlib import Path
from sys import argv

from colorama import Fore

from solve import TestFailedError


Status = Enum("Status", "PASSED FAILED ERRORED")


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
        results = pool.map(__run, modules)

    passed, failed, errors = 0, 0, 0
    for result in results:
        match result:
            case Status.PASSED: passed += 1
            case Status.FAILED: failed += 1
            case Status.ERRORED: errors += 1

    def color(color, n):
        return color if n > 0 else Fore.RESET

    print(
        Fore.RESET + f"Passed: {color(Fore.GREEN, passed) + str(passed) + Fore.RESET}, "
        f"failed: {color(Fore.YELLOW, failed) + str(failed) + Fore.RESET}, "
        f"errors: {color(Fore.RED, errors) + str(errors) + Fore.RESET}"
    )

    if failed or errors:
        exit(1)


def __build_module_paths(package):
    paths = Path(f"./{package}").glob(r"*[!_].py")
    return [f"{package}.{path.stem}" for path in paths]


def __run(module):
    try:
        import_module(module)

        print(Fore.GREEN + f"- {module} passed ✓")
        return Status.PASSED
    except TestFailedError as e:
        print(
            Fore.YELLOW
            + f"! {module} failed; expected: {e.expected!r}, got: {e.actual!r}"
        )
        return Status.FAILED
    except Exception as e:
        print(Fore.RED + f"! {module} failed to compile with error:", e)
        return Status.ERRORED


if __name__ == "__main__":
    __run_all(argv)
