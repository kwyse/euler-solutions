from functools import partial

from solve import solve


solve = partial(solve, data_dir="./advent_of_code/data")
