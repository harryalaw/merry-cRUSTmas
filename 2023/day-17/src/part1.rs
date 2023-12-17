use hashbrown::HashMap;
use std::{cmp::Reverse, collections::BinaryHeap};

#[tracing::instrument]
pub fn process(input: &str) -> usize {
    let grid = parse_grid(input);

    let mut distance_map: HashMap<u32, usize> = HashMap::new();

    let mut priority_q = BinaryHeap::new();
    let start_node_1 = Node {
        pos: (0, 0),
        direction: Direction::Right,
        minimum: 0,
    };
    let start_node_2 = Node {
        pos: (0, 0),
        direction: Direction::Right,
        minimum: 0,
    };

    priority_q.push(Reverse(start_node_1));
    priority_q.push(Reverse(start_node_2));
    let target = (grid.grid.len() - 1, grid.grid[0].len() - 1);

    while let Some(Reverse(node)) = priority_q.pop() {
        if node.pos == target {
            return node.minimum;
        }
        let key = create_key(&node);

        let previous = *distance_map.get(&(key)).unwrap_or(&usize::MAX);
        if node.minimum > previous {
            continue;
        }

        node.add_neighbours(&grid, &mut distance_map, &mut priority_q);
    }

    panic!("couldn't get to the end");
}

fn create_key(node: &Node) -> u32 {
    let (row, col) = node.pos;
    let direction_bytes = match node.direction {
        Direction::Up => 1,
        Direction::Right => 2,
        Direction::Down => 4,
        Direction::Left => 8,
    };
    u32::from_be_bytes([0, row as u8, col as u8, direction_bytes])
}

struct Grid {
    grid: Vec<Vec<usize>>,
}

fn parse_grid(input: &str) -> Grid {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).expect("It's a number") as usize)
                .collect()
        })
        .collect();

    Grid { grid }
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
        grid: &Grid,
        distance_map: &mut HashMap<u32, usize>,
        priority_q: &mut BinaryHeap<Reverse<Node>>,
    ) {
        let max_row = grid.grid.len();
        let max_col = grid.grid[0].len();

        let row = self.pos.0;
        let col = self.pos.1;

        for dir in Direction::iter() {
            if dir == self.direction || dir == self.direction.inverse() {
                continue;
            }
            let mut minimum = self.minimum;
            for steps in 1..=3 {
                let (row, col) = dir.travel(row, col, steps);

                if row >= max_row || col >= max_col {
                    continue;
                }
                minimum += grid.grid[row][col];

                let neighbour = Node {
                    pos: (row, col),
                    direction: dir,
                    minimum,
                };
                let key = create_key(&neighbour);
                let previous = *distance_map.get(&(key)).unwrap_or(&usize::MAX);

                if neighbour.minimum < previous {
                    distance_map.insert(key, neighbour.minimum);
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
        assert_eq!(102, process(input));
    }

    #[test]
    fn test_cmp() {
        let n1 = Node {
            pos: (1, 5),
            direction: Direction::Right,
            minimum: 20,
        };
        let smaller = Node {
            pos: (4, 2),
            direction: Direction::Down,
            minimum: 19,
        };

        let result = n1.cmp(&smaller);

        assert_eq!(std::cmp::Ordering::Greater, result);
    }
}
