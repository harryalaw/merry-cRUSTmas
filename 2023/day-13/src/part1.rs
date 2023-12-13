use std::fmt;

#[tracing::instrument]
pub fn process(_input: &str) -> usize {
    let puzzles = parse_input(_input);
    puzzles.iter().map(find_symmetry).sum()
}

fn find_symmetry(mirrors: &MirrorMaze) -> usize {
    // find horizontal symmetry
    for i in 1..mirrors.col_based.len() {
        if is_symmetric(&mirrors.col_based, i) {
            return i;
        }
    }

    // find vertical symmetry
    for j in 1..mirrors.row_based.len() {
        if is_symmetric(&mirrors.row_based, j) {
            return 100 * j;
        }
    }

    panic!("Not symmetric")
}

fn is_symmetric(array: &[Vec<char>], index: usize) -> bool {
    let mut hi = index;
    let mut lo = index;
    while hi < array.len() && lo > 0 {
        if array[hi] != array[lo - 1] {
            return false;
        }
        hi += 1;
        lo -= 1;
    }
    true
}

struct MirrorMaze {
    row_based: Vec<Vec<char>>,
    col_based: Vec<Vec<char>>,
}

impl fmt::Debug for MirrorMaze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut joined_rows = String::new();

        for row in &self.row_based[0..self.row_based.len()] {
            let row_string: String = row.iter().collect::<String>() + "\n";
            joined_rows.push_str(&row_string);
        }
        let mut joined_cols = String::new();

        for col in &self.col_based[0..self.col_based.len()] {
            let col_string: String = col.iter().collect::<String>() + "\n";
            joined_cols.push_str(&col_string);
        }

        write!(f, "\n{}\n{}\n", joined_rows, joined_cols)
    }
}

fn parse_input(input: &str) -> Vec<MirrorMaze> {
    input
        .split("\n\n")
        .map(|puzzle| {
            let row_based: Vec<Vec<char>> =
                puzzle.lines().map(|line| line.chars().collect()).collect();

            let mut col_based = Vec::with_capacity(row_based[0].len());
            for col in 0..row_based[0].len() {
                let mut new_col = Vec::with_capacity(row_based.len());
                for row in &row_based {
                    new_col.push(row[col]);
                }
                col_based.push(new_col);
            }

            MirrorMaze {
                row_based,
                col_based,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(405, process(input));
    }
}
