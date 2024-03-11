
#[tracing::instrument]
pub fn process(input: &str) -> usize {
    let (grid, start) = parse_input(input);

    let ans = count_positions(&grid, start, 26501365);

    ans
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

    let mut i = 1;
    while !positions.is_empty() {
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
        i+=1;
    }

    let inner_evens: usize = step_counts.iter().map(|row| row.iter().filter(|x| **x%2 == 0 && **x <= 65).count()).sum();
    let inner_odds: usize = step_counts.iter().map(|row| row.iter().filter(|x| **x%2 == 1 && **x <= 65).count()).sum();

    let outer_evens:usize = step_counts.iter().map(|row| row.iter().filter(|x| **x%2 == 0 && **x != usize::MAX && **x > 65).count()).sum();
    let outer_odds:usize = step_counts.iter().map(|row| row.iter().filter(|x| **x%2 == 1 && **x != usize::MAX && **x > 65).count()).sum();
    dbg!(inner_evens, inner_odds, outer_evens, outer_odds);

    let a_0 = inner_evens;
    let a_1 = inner_evens + 2 * outer_evens + 4 * inner_odds + 2 * outer_odds;
    let a_2 = 5 * inner_evens + 4 * inner_odds + 
    dbg!(a_0);
    dbg!(a_1);


    let n = 0;
    let a_0 = ((n+1)*(n+1)) * inner_odds + (n*n) * inner_evens - (n+1) * outer_odds + n * outer_evens;
    let n = 1;
    let a_1 = ((n+1)*(n+1)) * inner_odds + (n*n) * inner_evens - (n+1) * outer_odds + n * outer_evens;
    let n = 2;
    let a_2 = ((n+1)*(n+1)) * inner_odds + (n*n) * inner_evens - (n+1) * outer_odds + n * outer_evens;
    let n = 202300;
    let a_3 = ((n+1)*(n+1)) * inner_odds + (n*n) * inner_evens - (n+1) * outer_odds + n * outer_evens;

    dbg!(a_0, a_1, a_2, a_3);

    step_counts
        .iter()
        .map(|row| row.iter().filter(|x| **x%2 ==0 && **x <= 65).count())
        .sum()
}

struct Grid {
    height: usize,
    width: usize,
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn is_valid(&self, row: isize, col: isize) -> bool {
        let row_idx =
            (((row % self.height as isize) + self.height as isize) as usize) % self.height;
        let col_idx = (((col % self.width as isize) + self.width as isize) as usize) % self.width;

        self.grid[row_idx][col_idx] == '.'
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
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
        if self.col < grid.width - 1 && grid.grid[self.row][self.col + 1] == '.'{
            neighbours.push(Coord {
                row: self.row,
                col: self.col + 1,
            })
        };

        if self.row < grid.height - 1&& grid.grid[self.row + 1][self.col] == '.' {
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
    use rstest::rstest;

    #[rstest]
    #[case(6, 16)]
    #[case(10, 50)]
    #[case(50, 1594)]
    #[case(100, 6536)]
    #[case(500, 167004)]
    #[case(1000, 668697)]
    #[case(5000, 16733044)]
    fn test_process(#[case] iterations: usize, #[case] expected: usize) {
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

        let count = count_positions_using_interpolation(&grid, vec![start], iterations);
        assert_eq!(expected, count);
    }
}
