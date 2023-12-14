#[tracing::instrument]
pub fn process(input: &str) -> usize {
    let grid = parse_grid(input);

    let mut states = States::new(grid);

    let start = states.find_loop_start();
    let period = states.find_loop_period();
    let index = (1_000_000_000 - start) % period + start;

    states.scores[index].0
}

#[derive(Debug, Clone)]
struct States {
    scores: Vec<(usize, usize)>,
    grid: Grid,
}

impl States {
    fn new(grid: Grid) -> States {
        let scores = vec![score(&grid)];
        States { scores, grid }
    }

    fn value(&mut self, idx: usize) -> (usize, usize) {
        if idx < self.scores.len() - 1 {
            return self.scores[idx];
        }
        let mut temp_idx = self.scores.len() - 1;

        while temp_idx != idx {
            self.grid = spin_cycle(1, &self.grid);
            temp_idx += 1;
            self.scores.push(score(&self.grid));
        }

        self.scores[idx]
    }

    fn find_loop_start(&mut self) -> usize {
        let mut tortoise = 1;
        let mut hare = 2;

        while self.value(tortoise) != self.value(hare) {
            tortoise += 1;
            hare += 2;
        }

        tortoise = 0;
        while self.value(tortoise) != self.value(hare) {
            tortoise += 1;
            hare += 1;
        }

        tortoise
    }

    fn find_loop_period(&mut self) -> usize {
        let loop_start = self.find_loop_start();
        let mut tortoise = loop_start;
        let mut hare = loop_start + 1;
        let mut period = 1;
        while self.value(tortoise) != self.value(hare) {
            tortoise += 1;
            hare += 2;
            period += 1;
        }

        period
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Score {
    prev_index: usize,
    value_before: usize,
}

fn spin_cycle(count: usize, grid: &Grid) -> Grid {
    let mut grid = grid.to_owned();

    for _i in 0..count {
        grid = slide_north(&grid);
        grid = slide_west(&grid);
        grid = slide_south(&grid);
        grid = slide_east(&grid);
    }

    grid
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

fn slide_west(old_grid: &Grid) -> Grid {
    let mut new_grid = old_grid.grid.clone();

    for row in 0..old_grid.height {
        let mut next_gap = 0;
        for col in 0..old_grid.width {
            match old_grid.grid[row][col] {
                '.' => {
                    new_grid[row][col] = '.';
                }
                '#' => {
                    new_grid[row][col] = '#';
                    next_gap = col + 1;
                }
                'O' => {
                    new_grid[row][col] = '.';
                    new_grid[row][next_gap] = 'O';
                    next_gap += 1;
                    while next_gap < old_grid.width && new_grid[row][next_gap] == '#' {
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

fn slide_south(old_grid: &Grid) -> Grid {
    let mut new_grid = old_grid.grid.clone();

    for col in (0..old_grid.width).rev() {
        let mut next_gap = old_grid.width - 1;
        for row in (0..old_grid.height).rev() {
            match old_grid.grid[row][col] {
                '.' => {
                    new_grid[row][col] = '.';
                }
                '#' => {
                    new_grid[row][col] = '#';
                    if row > 0 {
                        next_gap = row - 1;
                    }
                }
                'O' => {
                    new_grid[row][col] = '.';
                    new_grid[next_gap][col] = 'O';
                    if row > 0 {
                        next_gap -= 1;
                        while next_gap > 0 && new_grid[next_gap][col] == '#' {
                            next_gap += 1;
                        }
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

fn slide_east(old_grid: &Grid) -> Grid {
    let mut new_grid = old_grid.grid.clone();

    for row in 0..old_grid.height {
        let mut next_gap = old_grid.width - 1;
        for col in (0..old_grid.width).rev() {
            match old_grid.grid[row][col] {
                '.' => {
                    new_grid[row][col] = '.';
                }
                '#' => {
                    new_grid[row][col] = '#';
                    if col > 0 {
                        next_gap = col - 1;
                    }
                }
                'O' => {
                    new_grid[row][col] = '.';
                    new_grid[row][next_gap] = 'O';
                    if next_gap > 0 {
                        next_gap -= 1;
                        while next_gap > 0 && new_grid[row][next_gap] == '#' {
                            next_gap += 1;
                        }
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

fn score(grid: &Grid) -> (usize, usize) {
    let mut north_total = 0;
    for row in 0..grid.height {
        let rocks = grid.grid[row].iter().filter(|x| *x == &'O').count();

        north_total += (grid.height - row) * rocks;
    }

    let mut west_total = 0;

    for col in 0..grid.width {
        let mut col_rocks = 0;
        for row in 0..grid.height {
            if grid.grid[row][col] == 'O' {
                col_rocks += 1;
            }
        }
        west_total += (grid.width - col) * col_rocks;
    }

    (north_total, west_total)
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
        assert_eq!(64, process(input));
    }

    #[test]
    fn test_spin_cycle_1() {
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

        let output = parse_grid(
            ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....",
        );

        assert_eq!(output, spin_cycle(1, &parse_grid(input)));
    }

    #[test]
    fn test_spin_cycle_2() {
        let input = parse_grid(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        );

        let output = parse_grid(
            ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O",
        );

        assert_eq!(output, spin_cycle(2, &input));
    }

    #[test]
    fn test_spin_cycle_3() {
        let input = parse_grid(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        );

        let output = parse_grid(
            ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O",
        );

        assert_eq!(output, spin_cycle(3, &input));
    }
}
