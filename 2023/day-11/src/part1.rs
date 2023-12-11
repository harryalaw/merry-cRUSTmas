#[tracing::instrument]
pub fn process(input: &str) -> usize {
    let (galaxy_rows, galaxy_cols) = parse_input(input);

    distances(&galaxy_rows) + distances(&galaxy_cols)
}

fn distances(coord: &[usize]) -> usize {
    let mut total = 0;
    for i in 0..coord.len() {
        for j in i+1..coord.len() {
            total += coord[j] - coord[i];
        }
    }
    total
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut galaxy_rows = Vec::new();
    let mut galaxy_cols = Vec::new();

    let mut empty_row_count = 0;
    for row in 0..grid.len() {
        let mut row_empty = true;
        for col in 0..grid[0].len() {
            if grid[row][col] == '#' {
                row_empty = false;
                galaxy_rows.push(row + empty_row_count);
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
                galaxy_cols.push(col + empty_col_count);
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

