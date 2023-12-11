#[tracing::instrument]
pub fn process(input: &str) -> usize {
    let (galaxies, empty_rows, empty_cols) = parse_input(input);

    let mut total = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let source = galaxies[i];
            let target = galaxies[j];
            total += distance(source, target, &empty_rows, &empty_cols);
        }
    }

    total
}

fn distance(
    source: (usize, usize),
    target: (usize, usize),
    empty_rows: &[usize],
    empty_cols: &[usize],
) -> usize {
    let start_x = source.0.min(target.0);
    let start_y = source.1.min(target.1);

    let end_x = source.0.max(target.0);
    let end_y = source.1.max(target.1);

    let rows = empty_rows[end_x]-empty_rows[start_x];
    let cols = empty_cols[end_y]-empty_cols[start_y];

    let x_distance = end_x - start_x + rows;
    let y_distance = end_y - start_y + cols;

    x_distance + y_distance
}

fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<usize>, Vec<usize>) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut galaxies = Vec::new();
    let mut empty_rows = Vec::new();

    let mut empty_row_count = 0;
    for row in 0..grid.len() {
        let mut row_empty = true;
        for col in 0..grid[0].len() {
            if grid[row][col] == '#' {
                row_empty = false;
                galaxies.push((row, col));
            }
        }
        if row_empty {
            empty_row_count += 1;
        }
        empty_rows.push(empty_row_count);
    }

    let mut empty_cols = Vec::new();
    let mut empty_col_count = 0;
    for col in 0..grid[0].len() {
        let mut col_empty = true;
        for row in &grid {
            if row[col] == '#' {
                col_empty = false;
            }
        }
        if col_empty {
            empty_col_count += 1;
        }
        empty_cols.push(empty_col_count);
    }

    (galaxies, empty_rows, empty_cols)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
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
        assert_eq!(374, process(input));
    }
}

