#[tracing::instrument]
pub fn process(_input: &str) -> usize {
    let grid: Vec<&[u8]> = _input.lines().map(|x| x.as_bytes()).collect();

    let mut count = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if check_for_x_mas(&grid, i, j) {
                count += 1;
            }
        }
    }

    count
}

fn check_for_x_mas(grid: &Vec<&[u8]>, i: usize, j: usize) -> bool {
    if i == 0 || j == 0 || i == grid.len() - 1 || j == grid[0].len() - 1 || grid[i][j] != b'A' {
        return false;
    }

    let corners = (
        grid[i - 1][j - 1],
        grid[i - 1][j + 1],
        grid[i + 1][j + 1],
        grid[i + 1][j - 1],
    );

    match corners {
        (b'M', b'M', b'S', b'S') => true,
        (b'S', b'M', b'M', b'S') => true,
        (b'S', b'S', b'M', b'M') => true,
        (b'M', b'S', b'S', b'M') => true,
        (_, _, _, _) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(9, process(input));
    }
}
