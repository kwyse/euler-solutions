from . import solve


@solve(expected=74, resource="2015_day1.txt")
def day1_part1(data):
    floor = 0
    for c in data:
        match c:
            case "(": floor += 1
            case ")": floor -= 1
        
    return floor


@solve(expected=1_795, resource="2015_day1.txt")
def day1_part2(data):
    floor = 0
    for i, c in enumerate(data, 1):
        match c:
            case "(": floor += 1
            case ")": floor -= 1
        
        if floor < 0:
            return i
