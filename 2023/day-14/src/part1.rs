#[tracing::instrument]
pub fn process(input: &str) -> usize {
    let grid = parse_grid(input);

    let grid = slide_north(&grid);

    score(&grid)
}

fn slide_north(old_grid: &Grid) -> Grid {
    let mut new_grid = old_grid.grid.clone();

    for col in 0..old_grid.width {
        let mut next_gap = 0;
        for row in 0..old_grid.height {
            match old_grid.grid[row][col] {
                '.' => {
                    new_grid[row][col] = '.';
                }
                '#' => {
                    new_grid[row][col] = '#';
                    next_gap = row + 1;
                }
                'O' => {
                    new_grid[row][col] = '.';
                    new_grid[next_gap][col] = 'O';
                    next_gap += 1;
                    while next_gap < old_grid.height && new_grid[next_gap][col] == '#' {
                        next_gap += 1;
                    }
                }
                _ => panic!("Unexpected item in the bagging area"),
            }
        }
    }

    Grid {
        height: old_grid.height,
        width: old_grid.width,
        grid: new_grid,
    }
}

fn score(grid: &Grid) -> usize {
    let mut total = 0;
    for row in 0..grid.height {
        let rocks = grid.grid[row].iter().filter(|x| *x == &'O').count();

        total += (grid.height - row) * rocks;
    }
    total
}

struct Grid {
    height: usize,
    width: usize,
    grid: Vec<Vec<char>>,
}

fn parse_grid(input: &str) -> Grid {
    let grid: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let height = grid.len();
    let width = grid[0].len();

    Grid {
        width,
        height,
        grid,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(136, process(input));
    }
}
