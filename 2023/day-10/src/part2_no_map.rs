#[tracing::instrument]
pub fn process(_input: &str) -> isize {
    let (start, initial_dir, map) = parse_input(_input);
    let (perimeter, corners) = traverse(map, start, initial_dir);
    let area = shoelace(corners);
    area + 1 - (perimeter / 2) as isize
}

fn traverse(
    pipe_grid: Vec<Vec<char>>,
    start: Coord,
    initial_dir: Direction,
) -> (usize, Vec<Coord>) {
    let mut pos = start;
    let mut dir = initial_dir;
    pos = pos.travel(&dir);
    let mut perimeter = 1;

    let mut corners = Vec::new();

    if is_corner(pipe_grid[start.row][start.col]) {
        corners.push(start);
    }

    while pos != start {
        let new_pipe = pipe_grid[pos.row][pos.col];
        if new_pipe == 'F' || new_pipe == '7' || new_pipe == 'L' || new_pipe == 'J' {
            corners.push(pos);
        }
        dir = new_dir(&dir, new_pipe);
        pos = pos.travel(&dir);
        perimeter += 1
    }

    corners.push(corners[0]);

    (perimeter, corners)
}

fn is_corner(c: char) -> bool {
    c == 'F' || c == '7' || c == 'L' || c == 'J'
}

fn shoelace(coords: Vec<Coord>) -> isize {
    let mut total: isize = 0;
    for i in 0..coords.len() - 1 {
        total += (coords[i].row * coords[i + 1].col) as isize
            - (coords[i].col * coords[i + 1].row) as isize;
    }
    total /= 2;
    isize::abs(total)
}

fn parse_input(input: &str) -> (Coord, Direction, Vec<Vec<char>>) {
    let mut start_pos = Coord { row: 0, col: 0 };

    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = grid.len();
    let width = grid[0].len();

    for i in 0..height {
        for j in 0..width {
            if grid[i][j] == 'S' {
                start_pos = Coord { row: i, col: j };

                break;
            }
        }
    }

    let (initial_dir, tile) = handle_start(&start_pos, &grid, height, width);
    grid[start_pos.row][start_pos.col] = tile;

    (start_pos, initial_dir, grid)
}

fn handle_start(
    start_pos: &Coord,
    grid: &[Vec<char>],
    height: usize,
    width: usize,
) -> (Direction, char) {
    let north_neighbour = if start_pos.row == 0 {
        '.'
    } else {
        grid[start_pos.row - 1][start_pos.col]
    };
    let south_neighbour = if start_pos.row == height - 1 {
        '.'
    } else {
        grid[start_pos.row + 1][start_pos.col]
    };
    let east_neighbour = if start_pos.col == 0 {
        '.'
    } else {
        grid[start_pos.row][start_pos.col - 1]
    };
    let west_neighbour = if start_pos.col == width - 1 {
        '.'
    } else {
        grid[start_pos.row][start_pos.col + 1]
    };

    let valid_north = valid_neighbour(Direction::North, north_neighbour);
    let valid_south = valid_neighbour(Direction::South, south_neighbour);
    let valid_east = valid_neighbour(Direction::East, east_neighbour);
    let valid_west = valid_neighbour(Direction::West, west_neighbour);

    match (valid_north, valid_south, valid_east, valid_west) {
        (true, true, false, false) => (Direction::North, '|'),
        (true, false, true, false) => (Direction::North, 'L'),
        (true, false, false, true) => (Direction::North, 'J'),
        (false, true, true, false) => (Direction::South, 'F'),
        (false, true, false, true) => (Direction::South, '7'),
        (false, false, true, true) => (Direction::East, '-'),
        _ => panic!("Failed to identify what the start pipe should be!"),
    }
}

fn valid_neighbour(dir: Direction, c: char) -> bool {
    match (dir, c) {
        (Direction::North, '|' | 'F' | '7') => true,
        (Direction::North, _) => false,
        (Direction::South, '|' | 'L' | 'J') => true,
        (Direction::South, _) => false,
        (Direction::East, '-' | 'F' | 'L') => true,
        (Direction::East, _) => false,
        (Direction::West, '-' | '7' | 'J') => true,
        (Direction::West, _) => false,
    }
}

fn new_dir(dir: &Direction, c: char) -> Direction {
    match (dir, c) {
        (Direction::North, '|') => Direction::North,
        (Direction::North, 'F') => Direction::East,
        (Direction::North, '7') => Direction::West,
        (Direction::South, '|') => Direction::South,
        (Direction::South, 'L') => Direction::East,
        (Direction::South, 'J') => Direction::West,
        (Direction::East, '-') => Direction::East,
        (Direction::East, '7') => Direction::South,
        (Direction::East, 'J') => Direction::North,
        (Direction::West, '-') => Direction::West,
        (Direction::West, 'L') => Direction::North,
        (Direction::West, 'F') => Direction::South,
        (_, _) => panic!("Invalid dir char combo {:?} {}", dir, c),
    }
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn travel(&self, direction: &Direction) -> Coord {
        match direction {
            Direction::North => Coord {
                row: self.row - 1,
                col: self.col,
            },
            Direction::South => Coord {
                row: self.row + 1,
                col: self.col,
            },
            Direction::East => Coord {
                row: self.row,
                col: self.col + 1,
            },
            Direction::West => Coord {
                row: self.row,
                col: self.col - 1,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!(4, process(input));
    }

    #[test]
    fn test_process2() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(8, process(input));
    }

    #[test]
    fn test_process3() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(10, process(input));
    }
}
