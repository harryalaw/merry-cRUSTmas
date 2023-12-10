#[tracing::instrument]
pub fn process(_input: &str) -> usize {
    let (start, map) = parse_input(_input);
    traverse(map, start)
}

fn traverse(pipe_grid: PipeGrid, start: Coord) -> usize {
    let mut visited = vec![vec![false; pipe_grid.width]; pipe_grid.height];
    let mut distance = 0;

    let mut curr_coords = vec![start];
    visited[start.row][start.col] = true;

    while !curr_coords.is_empty() {
        distance += 1;
        let mut next_coords = Vec::new();

        for coord in curr_coords {
            if let Some(nbrs) = &pipe_grid.grid[coord.row][coord.col] {
                for nbr in nbrs {
                    if !visited[nbr.row][nbr.col] {
                        next_coords.push(*nbr);
                        visited[nbr.row][nbr.col] = true;
                    }
                }
            }
        }

        curr_coords = next_coords
    }

    distance - 1
}

struct PipeGrid {
    width: usize,
    height: usize,
    grid: Vec<Vec<Option<Vec<Coord>>>>,
}

fn parse_input(input: &str) -> (Coord, PipeGrid) {
    let mut start_pos = Coord { row: 0, col: 0 };
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let mut grid: Vec<Vec<Option<Vec<Coord>>>> = vec![vec![None; width]; height];
    input
        .lines()
        .enumerate()
        .flat_map(|(row, x)| x.chars().enumerate().map(move |(col, x)| (row, col, x)))
        .filter(|(_row, _col, x)| x != &'.')
        .map(|(row, col, x)| (x, Coord::new(row, col)))
        .for_each(|(symbol, coord)| {
            let mut vec = Vec::with_capacity(2);
            let neighbours = match symbol {
                //is a vertical pipe connecting north and south.
                '|' => {
                    if coord.row > 0 {
                        vec.push(coord.neighbour(Direction::North));
                    }
                    if coord.row < height - 1 {
                        vec.push(coord.neighbour(Direction::South));
                    }
                    vec
                }
                '-' => {
                    if coord.col > 0 {
                        vec.push(coord.neighbour(Direction::West));
                    }
                    if coord.col < width - 1 {
                        vec.push(coord.neighbour(Direction::East));
                    }
                    vec
                }
                //is a 90-degree bend connecting north and east.
                'L' => {
                    if coord.row > 0 {
                        vec.push(coord.neighbour(Direction::North));
                    }
                    if coord.col < width - 1 {
                        vec.push(coord.neighbour(Direction::East));
                    }
                    vec
                }
                //is a 90-degree bend connecting north and west.
                'J' => {
                    if coord.row > 0 {
                        vec.push(coord.neighbour(Direction::North));
                    }
                    if coord.col > 0 {
                        vec.push(coord.neighbour(Direction::West));
                    }
                    vec
                }
                //is a 90-degree bend connecting south and west.
                '7' => {
                    if coord.col > 0 {
                        vec.push(coord.neighbour(Direction::West));
                    }
                    if coord.row < height - 1 {
                        vec.push(coord.neighbour(Direction::South));
                    }
                    vec
                }
                //is a 90-degree bend connecting south and east.
                'F' => {
                    if coord.col < width - 1 {
                        vec.push(coord.neighbour(Direction::East));
                    }
                    if coord.row < height - 1 {
                        vec.push(coord.neighbour(Direction::South));
                    }
                    vec
                }
                //is the starting position of the animal;
                //there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
                'S' => {
                    start_pos = Coord {
                        row: coord.row,
                        col: coord.col,
                    };
                    if coord.row > 0 {
                        if let Some(north) = input
                            .lines()
                            .nth(coord.row - 1)
                            .unwrap()
                            .chars()
                            .nth(coord.col)
                        {
                            if north == '|' || north == 'F' || north == '7' {
                                vec.push(coord.neighbour(Direction::North));
                            }
                        }
                    }
                    if coord.row < (height - 1) {
                        if let Some(south) = input
                            .lines()
                            .nth(coord.row + 1)
                            .unwrap()
                            .chars()
                            .nth(coord.col)
                        {
                            if south == '|' || south == 'J' || south == 'L' {
                                vec.push(coord.neighbour(Direction::South));
                            }
                        }
                    }
                    if coord.col < (width - 1) {
                        if let Some(east) = input
                            .lines()
                            .nth(coord.row)
                            .unwrap()
                            .chars()
                            .nth(coord.col + 1)
                        {
                            if east == '-' || east == 'J' || east == '7' {
                                vec.push(coord.neighbour(Direction::East));
                            }
                        }
                    }
                    if coord.col > 0 {
                        if let Some(west) = input
                            .lines()
                            .nth(coord.row)
                            .unwrap()
                            .chars()
                            .nth(coord.col - 1)
                        {
                            if west == '-' || west == 'L' || west == 'F' {
                                vec.push(coord.neighbour(Direction::West))
                            }
                        }
                    }
                    vec
                }
                _ => panic!("Unexpected symbol detected {}", symbol),
            };

            grid[coord.row][coord.col] = Some(neighbours);
        });

    (
        start_pos,
        PipeGrid {
            width,
            height,
            grid,
        },
    )
}

enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Copy, Clone)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn new(row: usize, col: usize) -> Coord {
        Coord { row, col }
    }

    fn neighbour(&self, dir: Direction) -> Coord {
        match dir {
            Direction::North => Coord::new(self.row - 1, self.col),
            Direction::South => Coord::new(self.row + 1, self.col),
            Direction::East => Coord::new(self.row, self.col + 1),
            Direction::West => Coord::new(self.row, self.col - 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!(4, process(input));
    }

    #[test]
    fn test_process2() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(8, process(input));
    }
}
