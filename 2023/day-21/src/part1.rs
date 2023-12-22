#[tracing::instrument]
pub fn process(input: &str) -> usize {
    let (grid, start) = parse_input(input);

    count_positions(&grid, start, 64)
}

fn parse_input(input: &str) -> (Grid, Coord) {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = grid.len();
    let width = grid[0].len();

    let mut start = Coord { row: 0, col: 0 };
    for row in 0..height {
        for col in 0..width {
            if grid[row][col] == 'S' {
                start.row = row;
                start.col = col;
                grid[row][col] = '.';
                break;
            }
        }
    }

    (
        Grid {
            grid,
            height,
            width,
        },
        start,
    )
}

fn count_positions(grid: &Grid, start: Coord, iterations: usize) -> usize {
    let mut positions = Vec::new();

    let mut step_counts: Vec<Vec<usize>> = grid
        .grid
        .iter()
        .map(|row| row.iter().map(|_| usize::MAX).collect())
        .collect();

    step_counts[start.row][start.col] = 0;
    positions.push(start);

    for i in 1..=iterations {
        let mut next_positions = Vec::new();
        for pos in positions {
            let neighbours = pos.get_neighbours(&grid);
            for neighbour in neighbours {
                if i < step_counts[neighbour.row][neighbour.col] {
                    step_counts[neighbour.row][neighbour.col] = i;
                    next_positions.push(neighbour);
                }
            }
        }

        positions = next_positions;
    }

    step_counts
        .iter()
        .map(|row| row.iter().filter(|x| **x%2 ==0 && **x <= iterations).count())
        .sum()
}

struct Grid {
    height: usize,
    width: usize,
    grid: Vec<Vec<char>>,
}

#[derive(Debug)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn get_neighbours(&self, grid: &Grid) -> Vec<Coord> {
        let mut neighbours = Vec::new();
        if self.row > 0 && grid.grid[self.row - 1][self.col] == '.' {
            neighbours.push(Coord {
                row: self.row - 1,
                col: self.col,
            })
        }
        if self.col > 0 && grid.grid[self.row][self.col - 1] == '.' {
            neighbours.push(Coord {
                row: self.row,
                col: self.col - 1,
            })
        }
        if self.col < grid.width && grid.grid[self.row][self.col + 1] == '.'{
            neighbours.push(Coord {
                row: self.row,
                col: self.col + 1,
            })
        };

        if self.row < grid.height && grid.grid[self.row + 1][self.col] == '.' {
            neighbours.push(Coord {
                row: self.row + 1,
                col: self.col,
            })
        }
        neighbours
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let (grid, start) = parse_input(
            "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........",
        );

        let count = count_positions(&grid, start, 6);
        assert_eq!(16, count);
    }
}
