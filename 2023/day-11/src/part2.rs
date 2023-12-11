#[tracing::instrument]
pub fn process(input: &str) -> usize {
    solve(input, 1_000_000)
}

fn solve(input: &str, scaling_factor: usize) -> usize {
    let (galaxy_rows, galaxy_cols) = parse_input(input, scaling_factor);

    distances(&galaxy_rows) + distances(&galaxy_cols)
}

fn distances(coord: &[usize]) -> usize {
    let mut total = 0;
    let mut previous = 0;

    for (i, value) in coord.iter().enumerate() {
        total += (i) * value - previous;
        previous += value;
    }

    total
}

fn parse_input(input: &str, scaling_factor: usize) -> (Vec<usize>, Vec<usize>) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut galaxy_rows = Vec::new();
    let mut galaxy_cols = Vec::new();

    let mut empty_row_count = 0;
    for row in 0..grid.len() {
        let mut row_empty = true;
        for col in 0..grid[0].len() {
            if grid[row][col] == '#' {
                row_empty = false;
                galaxy_rows.push(row + (scaling_factor - 1) * empty_row_count);
            }
        }
        if row_empty {
            empty_row_count += 1;
        }
    }

    let mut empty_col_count = 0;
    for col in 0..grid[0].len() {
        let mut col_empty = true;
        for row in &grid {
            if row[col] == '#' {
                galaxy_cols.push(col + (scaling_factor - 1) * empty_col_count);
                col_empty = false;
            }
        }
        if col_empty {
            empty_col_count += 1;
        }
    }

    (galaxy_rows, galaxy_cols)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_10() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(1030, solve(input, 10));
    }

    #[test]
    fn test_process_100() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(8410, solve(input, 100));
    }
}
