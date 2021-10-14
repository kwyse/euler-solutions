from . import solve


@solve(expected=1_588_178, resource="2015_day2.txt")
def part1(data):
    def surface_area(l, w, h):
        smallest_side = l * w * h / max(l, w, h)
        return 2 * l * w + 2 * w * h + 2 * h * l + smallest_side

    dimensions = [gift.split("x") for gift in data.splitlines()]
    return sum([surface_area(int(l), int(w), int(h)) for l, w, h in dimensions])


@solve(expected=3_783_758, resource="2015_day2.txt")
def part2(data):
    def smallest_perimeter(l, w, h):
        return 2 * l + 2 * w + 2 * h - 2 * max(l, w, h)

    def volume(l, w, h):
        return l * w * h

    def ribbon_needed(l, w, h):
        return smallest_perimeter(l, w, h) + volume(l, w, h)

    dimensions = [gift.split("x") for gift in data.splitlines()]
    return sum([ribbon_needed(int(l), int(w), int(h)) for l, w, h in dimensions])
