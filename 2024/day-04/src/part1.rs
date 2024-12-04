#[tracing::instrument]
pub fn process(_input: &str) -> usize {
    let grid: Vec<&[u8]> = _input.lines().map(|x| x.as_bytes()).collect();

    let mut count = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            count += check_for_xmas(&grid, i, j)
        }
    }

    count
}

fn check_for_xmas(grid: &Vec<&[u8]>, i: usize, j: usize) -> usize {
    if grid[i][j] != b'X' {
        return 0;
    };

    let mut count = 0;

    let height = grid.len();
    let width = grid[0].len();

    if i < height - 3 && grid[i + 1][j] == b'M' && grid[i + 2][j] == b'A' && grid[i + 3][j] == b'S'
    {
        count += 1;
    }
    if i < height - 3
        && j < width - 3
        && grid[i + 1][j + 1] == b'M'
        && grid[i + 2][j + 2] == b'A'
        && grid[i + 3][j + 3] == b'S'
    {
        count += 1;
    }
    if j < width - 3 && grid[i][j + 1] == b'M' && grid[i][j + 2] == b'A' && grid[i][j + 3] == b'S' {
        count += 1;
    }
    if i > 2
        && j < width - 3
        && grid[i - 1][j + 1] == b'M'
        && grid[i - 2][j + 2] == b'A'
        && grid[i - 3][j + 3] == b'S'
    {
        count += 1;
    }
    if i >= 3 && grid[i - 1][j] == b'M' && grid[i - 2][j] == b'A' && grid[i - 3][j] == b'S' {
        count += 1;
    }
    if i >= 3
        && j >= 3
        && grid[i - 1][j - 1] == b'M'
        && grid[i - 2][j - 2] == b'A'
        && grid[i - 3][j - 3] == b'S'
    {
        count += 1;
    }
    if j >= 3 && grid[i][j - 1] == b'M' && grid[i][j - 2] == b'A' && grid[i][j - 3] == b'S' {
        count += 1;
    }
    if i < height - 3
        && j >= 3
        && grid[i + 1][j - 1] == b'M'
        && grid[i + 2][j - 2] == b'A'
        && grid[i + 3][j - 3] == b'S'
    {
        count += 1;
    }

    count
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
        assert_eq!(18, process(input));
    }
}
