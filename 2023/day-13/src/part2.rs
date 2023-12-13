use rayon::prelude::*;

#[tracing::instrument]
pub fn process(_input: &str) -> usize {
    let puzzles = parse_input(_input);
    puzzles.iter().map(find_near_symmetry).sum()
}

fn find_near_symmetry(mirrors: &MirrorMaze) -> usize {
    // find near horizontal symmetry
    for i in 1..mirrors.col_based.len() {
        if is_nearly_symmetric(&mirrors.col_based, i) {
            return i;
        }
    }

    // find near vertical symmetry
    for j in 1..mirrors.row_based.len() {
        if is_nearly_symmetric(&mirrors.row_based, j) {
            return 100 * j;
        }
    }

    panic!("No alternate found")
}

fn is_nearly_symmetric(array: &[Vec<char>], index: usize) -> bool {
    let mut hi = index;
    let mut lo = index;
    let mut diffs = 0;

    while hi < array.len() && lo > 0 && diffs < 2 {
        let line_diffs = count_diffs(&array[hi], &array[lo - 1]);
        if line_diffs > 1 {
            return false;
        }
        diffs += line_diffs;
        hi += 1;
        lo -= 1;
    }
    diffs == 1
}

fn count_diffs(a: &[char], b: &[char]) -> usize {
    let mut diffs = 0;
    for i in 0..a.len() {
        if a[i] != b[i] {
            diffs += 1
        }
    }
    diffs
}

struct MirrorMaze {
    row_based: Vec<Vec<char>>,
    col_based: Vec<Vec<char>>,
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
        assert_eq!(400, process(input));
    }
}
