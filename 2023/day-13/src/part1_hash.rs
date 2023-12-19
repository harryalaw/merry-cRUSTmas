use rayon::prelude::*;

#[tracing::instrument]
pub fn process(input: &str) -> usize {
    let puzzles: Vec<&str> = input.split("\n\n").collect();

    puzzles
        .par_iter()
        .map(|puzzle| {
            let grid: Vec<Vec<char>> = puzzle.lines().map(|line| line.chars().collect()).collect();
            let rows: Vec<u64> = grid.iter().map(|line| hash(line)).collect();

            let mut cols = Vec::with_capacity(grid[0].len());
            for col in 0..grid[0].len() {
                let mut new_col = Vec::with_capacity(rows.len());
                for row in &grid {
                    new_col.push(row[col]);
                }
                cols.push(hash(&new_col));
            }

            find_symmetry(&MirrorMaze { rows, cols })
        })
        .sum()
}

fn find_symmetry(mirrors: &MirrorMaze) -> usize {
    // find horizontal symmetry
    for i in 1..mirrors.cols.len() {
        if is_symmetric(&mirrors.cols, i) {
            return i;
        }
    }

    // find vertical symmetry
    for j in 1..mirrors.rows.len() {
        if is_symmetric(&mirrors.rows, j) {
            return 100 * j;
        }
    }

    panic!("Not symmetric")
}

fn is_symmetric(array: &[u64], index: usize) -> bool {
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
    rows: Vec<u64>,
    cols: Vec<u64>,
}

fn hash(line: &[char]) -> u64 {
    let mut hash = 0;
    for char in line {
        hash *= 2;
        if char == &'#' {
            hash += 1
        }
    }
    hash
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
