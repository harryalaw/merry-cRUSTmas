#[tracing::instrument]
pub fn process(input: &str) -> usize {
    solve(input, 1_000_000)
}

fn solve(input: &str, scaling_factor: usize) -> usize {
    let (galaxies, empty_rows, empty_cols) = parse_input(input);

    let mut total = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let source = galaxies[i];
            let target = galaxies[j];
            total += distance(source, target, &empty_rows, &empty_cols, scaling_factor);
        }
    }

    total
}

fn distance(
    source: (usize, usize),
    target: (usize, usize),
    empty_rows: &[usize],
    empty_cols: &[usize],
    scaling_factor: usize,
) -> usize {
    let start_x = source.0.min(target.0);
    let start_y = source.1.min(target.1);

    let end_x = source.0.max(target.0);
    let end_y = source.1.max(target.1);

    let rows = empty_rows
        .iter()
        .filter(|x| &start_x < *x && *x < &end_x)
        .count();
    let cols = empty_cols
        .iter()
        .filter(|y| &start_y < *y && *y < &end_y)
        .count();

    let x_distance = end_x - start_x + (scaling_factor - 1) * rows;
    let y_distance = end_y - start_y + (scaling_factor - 1) *cols;

    x_distance + y_distance
}

fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<usize>, Vec<usize>) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut galaxies = Vec::new();
    let mut empty_rows = Vec::new();
    let mut empty_cols = Vec::new();
    for row in 0..grid.len() {
        let mut row_empty = true;
        for col in 0..grid[0].len() {
            if grid[row][col] == '#' {
                row_empty = false;
                galaxies.push((row, col));
            }
        }
        if row_empty {
            empty_rows.push(row);
        }
    }

    for col in 0..grid[0].len() {
        let mut col_empty = true;
        for row in &grid {
            if row[col] == '#' {
                col_empty = false;
            }
        }
        if col_empty {
            empty_cols.push(col);
        }
    }

    (galaxies, empty_rows, empty_cols)
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
