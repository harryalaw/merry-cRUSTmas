#[tracing::instrument]
pub fn process(input: &str) -> isize {
    let commands = parse_commands(input);
    let mut corners: Vec<Point> = Vec::new();
    let mut pos = Point { x: 0, y: 0 };
    let mut perimeter: isize = 0;
    corners.push(pos);
    commands.iter().for_each(|command| {
        pos = pos.travel(command.distance, &command.direction);
        perimeter += command.distance;
        corners.push(pos);
    });
    shoelace_formula(&corners) + perimeter / 2 + 1
}

fn parse_commands(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_ascii_whitespace().collect();
            let direction = match parts[0] {
                "U" => Direction::Up,
                "L" => Direction::Left,
                "D" => Direction::Down,
                "R" => Direction::Right,
                _ => panic!("Invalid direction str: {}", parts[0]),
            };
            let distance = parts[1].parse::<isize>().expect("It's a number");

            Command {
                direction,
                distance,
            }
        })
        .collect()
}

fn shoelace_formula(corners: &[Point]) -> isize {
    let mut total = 0;
    for i in 0..corners.len() - 1 {
        total += corners[i].cross(&corners[i + 1]);
    }

    total /= 2;
    isize::abs(total)
}

#[derive(Copy, Clone, Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn travel(&self, distance: isize, dir: &Direction) -> Point {
        match dir {
            Direction::Up => Point {
                x: self.x - distance,
                y: self.y,
            },
            Direction::Left => Point {
                x: self.x,
                y: self.y - distance,
            },
            Direction::Down => Point {
                x: self.x + distance,
                y: self.y,
            },
            Direction::Right => Point {
                x: self.x,
                y: self.y + distance,
            },
        }
    }

    fn cross(&self, other: &Point) -> isize {
        self.x * other.y - self.y * other.x
    }
}

struct Command {
    direction: Direction,
    distance: isize,
}

enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!(62, process(input));
    }
}
