use rayon::prelude::*;

#[tracing::instrument]
pub fn process(_input: &str) -> usize {
    let puzzles = parse_input(_input);
    puzzles.iter().map(find_near_symmetry).sum()
}

fn find_near_symmetry(mirrors: &MirrorMaze) -> usize {
    // find near horizontal symmetry
    for i in 1..mirrors.cols.len() {
        if is_nearly_symmetric(&mirrors.cols, i) {
            return i;
        }
    }

    // find near vertical symmetry
    for j in 1..mirrors.rows.len() {
        if is_nearly_symmetric(&mirrors.rows, j) {
            return 100 * j;
        }
    }

    panic!("No alternate found")
}

fn is_nearly_symmetric(array: &[u64], index: usize) -> bool {
    let mut hi = index;
    let mut lo = index;
    let mut diffs = 0;

    while hi < array.len() && lo > 0 && diffs < 2 {
        let comparison = compare(array[hi], array[lo - 1]);
        match (diffs, comparison) {
            (_, Comparison::Equal) => {}
            (0, Comparison::OneDiff) => diffs += 1,
            (_, Comparison::OneDiff | Comparison::ManyDiffs) => return false,
        }

        hi += 1;
        lo -= 1;
    }
    diffs == 1
}

struct MirrorMaze {
    rows: Vec<u64>,
    cols: Vec<u64>,
}

fn parse_input(input: &str) -> Vec<MirrorMaze> {
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

            MirrorMaze { rows, cols }
        })
        .collect()
}

enum Comparison {
    Equal,
    OneDiff,
    ManyDiffs,
}

/*
  a,b are binary representations of the lines.
  If a == b then they represent the same line so are equal.
  If a differs from b by a power of two then they have one difference
   -> Represents flipping a . to a #
  Can get the diff of them with xor
  Then can use (a & (a-1)) to tell if it's a power of 2
*/
fn compare(a: u64, b: u64) -> Comparison {
    let xor = a ^ b;
    if a == b {
        Comparison::Equal
    } else {
        match xor & (xor - 1) == 0 {
            true => Comparison::OneDiff,
            false => Comparison::ManyDiffs,
        }
    }
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
        assert_eq!(400, process(input));
    }
}
