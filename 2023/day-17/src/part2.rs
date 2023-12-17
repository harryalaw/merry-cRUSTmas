use std::{cmp::Reverse, collections::BinaryHeap};

#[tracing::instrument]
pub fn process(input: &str) -> usize {
    let grid = parse_grid(input);

    let mut distance_map = FakeMap::new(grid.len(), grid[0].len());

    let mut priority_q = BinaryHeap::new();
    let start_node_1 = Node {
        pos: (0, 0),
        direction: Direction::Right,
        minimum: 0,
    };
    let start_node_2 = Node {
        pos: (0, 0),
        direction: Direction::Down,
        minimum: 0,
    };

    priority_q.push(Reverse(start_node_1));
    priority_q.push(Reverse(start_node_2));

    let target = (grid.len() - 1, grid[0].len() - 1);

    while let Some(Reverse(node)) = priority_q.pop() {
        if node.pos == target {
            return node.minimum;
        }

        let (row, col) = node.pos;
        let dir = node.direction;

        let previous = distance_map.get(row, col, dir);
        if node.minimum > previous {
            continue;
        }

        node.add_neighbours(&grid, &mut distance_map, &mut priority_q);
    }

    panic!("couldn't get to the end");
}

fn parse_grid(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).expect("It's a number") as usize)
                .collect()
        })
        .collect()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn iter() -> impl Iterator<Item = Direction> {
        [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
        .iter()
        .copied()
    }

    fn inverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }

    fn travel(&self, row: usize, col: usize, steps: usize) -> (usize, usize) {
        match self {
            Direction::Up => ((row as isize - steps as isize) as usize, col),
            Direction::Down => (row + steps, col),
            Direction::Left => (row, (col as isize - steps as isize) as usize),
            Direction::Right => (row, col + steps),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    pos: (usize, usize),
    direction: Direction,
    minimum: usize,
}

impl Node {
    fn add_neighbours(
        &self,
        grid: &[Vec<usize>],
        distance_map: &mut FakeMap,
        priority_q: &mut BinaryHeap<Reverse<Node>>,
    ) {
        let max_row = grid.len();
        let max_col = grid[0].len();

        let row = self.pos.0;
        let col = self.pos.1;

        for dir in Direction::iter() {
            if dir == self.direction || dir == self.direction.inverse() {
                continue;
            }
            let mut minimum = self.minimum;
            for steps in 1..=10 {
                let (row, col) = dir.travel(row, col, steps);

                if row >= max_row || col >= max_col {
                    continue;
                }
                minimum += grid[row][col];
                if steps < 4 {
                    continue;
                }

                let neighbour = Node {
                    pos: (row, col),
                    direction: dir,
                    minimum,
                };
                let previous = distance_map.get(row, col, dir);

                if neighbour.minimum < previous {
                    distance_map.insert(row, col, dir, neighbour.minimum);
                    priority_q.push(Reverse(neighbour));
                }
            }
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.minimum.cmp(&other.minimum)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct FakeMap {
    distances: Vec<Vec<[Option<usize>; 4]>>,
}

impl FakeMap {
    fn new(height: usize, width: usize) -> FakeMap {
        let mut distances = Vec::with_capacity(height);
        for _ in 0..height {
            let mut row = Vec::with_capacity(width);
            for _ in 0..width {
                row.push([None, None, None, None]);
            }
            distances.push(row);
        }

        FakeMap { distances }
    }

    fn get(&self, row: usize, col: usize, direction: Direction) -> usize {
        let cell = self.distances[row][col];
        match direction {
            Direction::Up => cell[0].unwrap_or(usize::MAX),
            Direction::Right => cell[1].unwrap_or(usize::MAX),
            Direction::Down => cell[2].unwrap_or(usize::MAX),
            Direction::Left => cell[3].unwrap_or(usize::MAX),
        }
    }

    fn insert(&mut self, row: usize, col: usize, direction: Direction, value: usize) {
        let cell = &mut self.distances[row][col];

        match direction {
            Direction::Up => cell[0] = Some(value),
            Direction::Right => cell[1] = Some(value),
            Direction::Down => cell[2] = Some(value),
            Direction::Left => cell[3] = Some(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        assert_eq!(94, process(input));
    }

    #[test]
    fn test_process_sad_grid() {
        let input = "111111111111
999999999991
999999999991
999999999991
999999999991";
        assert_eq!(71, process(input));
    }
}
