from . import solve


class Entity():
    def __init__(self, x: int, y: int) -> None:
        self.x = x
        self.y = y

    def move(self, direction: str):
        match direction:
            case "^": self.y += 1
            case "v": self.y -= 1
            case ">": self.x += 1
            case "<": self.x -= 1
    
    def as_tuple(self) -> tuple[int, int]:
        return (self.x, self.y)


@solve(expected=2_592, resource="2015_day3.txt")
def part1(data):
    santa = Entity(0, 0)

    visited = {santa.as_tuple()}
    for direction in data:
        santa.move(direction)
        visited.add(santa.as_tuple())

    return len(visited)


@solve(expected=2_360, resource="2015_day3.txt")
def part2(data):
    santa = Entity(0, 0)
    robo = Entity(0, 0)

    visited = {santa.as_tuple()}
    for i, direction in enumerate(data):
        if i % 2:
            robo.move(direction)
        else:
            santa.move(direction)
        
        visited.add(santa.as_tuple())
        visited.add(robo.as_tuple())

    return len(visited)
