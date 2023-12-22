use hashbrown::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> usize {
    let (grid, start) = parse_input(input);

    let ans = count_positions_using_interpolation(&grid, vec![start], 26501365);

    ans
}

fn parse_input(input: &str) -> (Grid, Coord) {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = grid.len();
    let width = grid[0].len();

    let mut start = Coord { row: 0, col: 0 };
    for row in 0..height {
        for col in 0..width {
            if grid[row][col] == 'S' {
                start.row = row as isize;
                start.col = col as isize;
                grid[row][col] = '.';
                break;
            }
        }
    }

    (
        Grid {
            grid,
            height,
            width,
        },
        start,
    )
}

fn count_positions_using_interpolation(grid: &Grid, start: Vec<Coord>, iterations: usize) -> usize {
    let mut positions = start.clone();

    let mut step_counts: HashMap<Coord, usize> = HashMap::new();

    for coord in positions.iter() {
        step_counts.insert(coord.clone(), 0);
    }

    let mut quadratic_values = [0, 0, 0];
    let mut next_index = 0;

    for i in 1..=iterations {
        let mut next_positions = Vec::new();
        for pos in positions {
            let neighbours = pos.get_neighbours(&grid);
            for neighbour in neighbours {
                if step_counts.get(&neighbour).is_none() {
                    step_counts.insert(neighbour, i);
                    next_positions.push(neighbour);
                }
            }
        }

        positions = next_positions;

        if i % 131 == 65 {
            let parity = i % 2;
            let counts = step_counts
                .iter()
                .map(|(_coord, value)| *value)
                .filter(|value| *value % 2 == parity && *value <= iterations)
                .count();
            quadratic_values[next_index] = counts;
            next_index += 1;

            if next_index == quadratic_values.len() {
                return interpolate(quadratic_values, iterations / 131);
            }
        }
    }
    0
}

// evaluate the quadratic f(x) = ax^2 + bx + c using the first 3 values of its function
fn interpolate(quadratic_values: [usize; 3], x: usize) -> usize {
    let c = quadratic_values[0];
    let a = (quadratic_values[2] - 2 * quadratic_values[1] + quadratic_values[0]) / 2;
    let b = quadratic_values[1] - c - a;

    return a * x * x + b * x + c;
}

struct Grid {
    height: usize,
    width: usize,
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn is_valid(&self, row: isize, col: isize) -> bool {
        let row_idx =
            (((row % self.height as isize) + self.height as isize) as usize) % self.height;
        let col_idx = (((col % self.width as isize) + self.width as isize) as usize) % self.width;

        self.grid[row_idx][col_idx] == '.'
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Coord {
    row: isize,
    col: isize,
}

impl Coord {
    fn get_neighbours(&self, grid: &Grid) -> Vec<Coord> {
        let mut neighbours = Vec::new();
        if grid.is_valid(self.row - 1, self.col) {
            neighbours.push(Coord {
                row: self.row - 1,
                col: self.col,
            })
        }
        if grid.is_valid(self.row, self.col - 1) {
            neighbours.push(Coord {
                row: self.row,
                col: self.col - 1,
            })
        }
        if grid.is_valid(self.row, self.col + 1) {
            neighbours.push(Coord {
                row: self.row,
                col: self.col + 1,
            })
        };

        if grid.is_valid(self.row + 1, self.col) {
            neighbours.push(Coord {
                row: self.row + 1,
                col: self.col,
            })
        }
        neighbours
    }
}
