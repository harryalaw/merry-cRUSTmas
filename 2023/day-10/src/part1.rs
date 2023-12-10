use std::collections::{HashMap, HashSet};

#[tracing::instrument]
pub fn process(_input: &str) -> usize {
    let (start, map) = parse_input(_input);
    traverse(map, start)
}

fn traverse(map: HashMap<usize, Vec<Coord>>, start: Coord) -> usize {
    let mut visited = HashSet::<usize>::new();
    let mut distance = 0;

    let mut curr_coords = vec![start];
    visited.insert(start.hash());

    while !curr_coords.is_empty() {
        distance += 1;
        let mut next_coords = Vec::new();

        for coord in curr_coords {

            let nbrs = map.get(&coord.hash()).expect("Should be in map");
            for nbr in nbrs {
                if !visited.contains(&nbr.hash()) {
                    next_coords.push(*nbr);
                    visited.insert(nbr.hash());
                }
            }
        }

        curr_coords = next_coords
    }

    distance - 1
}

fn parse_input(input: &str) -> (Coord, HashMap<usize, Vec<Coord>>) {
    let mut start_pos = Coord { row: 0, col: 0 };
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let pipes: Vec<(char, Coord)> = input
        .lines()
        .enumerate()
        .flat_map(|(row, x)| x.chars().enumerate().map(move |(col, x)| (row, col, x)))
        .filter(|(_row, _col, x)| x != &'.')
        .map(|(row, col, x)| {
            (
                x,
                Coord {
                    row: row.try_into().unwrap(),
                    col: col.try_into().unwrap(),
                },
            )
        })
        .collect();

    let mut out_map = HashMap::new();

    pipes.iter().for_each(|(symbol, coord)| {
        let hash_value = coord.hash();
        let neighbours = match *symbol {
            //is a vertical pipe connecting north and south.
            '|' => vec![
                Coord {
                    row: coord.row - 1,
                    col: coord.col,
                },
                Coord {
                    row: coord.row + 1,
                    col: coord.col,
                },
            ],
            //is a horizontal pipe connecting east and west.
            '-' => vec![
                Coord {
                    row: coord.row,
                    col: coord.col - 1,
                },
                Coord {
                    row: coord.row,
                    col: coord.col + 1,
                },
            ],
            //is a 90-degree bend connecting north and east.
            'L' => vec![
                Coord {
                    row: coord.row - 1,
                    col: coord.col,
                },
                Coord {
                    row: coord.row,
                    col: coord.col + 1,
                },
            ],
            //is a 90-degree bend connecting north and west.
            'J' => {
                if coord.row > 0 {
                    vec![
                        Coord {
                            row: coord.row - 1,
                            col: coord.col,
                        },
                        Coord {
                            row: coord.row,
                            col: coord.col - 1,
                        },
                    ]
                } else {
                    vec![Coord {
                        row: coord.row,
                        col: coord.col - 1,
                    }]
                }
            }
            //is a 90-degree bend connecting south and west.
            '7' => vec![
                Coord {
                    row: coord.row + 1,
                    col: coord.col,
                },
                Coord {
                    row: coord.row,
                    col: coord.col - 1,
                },
            ],
            //is a 90-degree bend connecting south and east.
            'F' => vec![
                Coord {
                    row: coord.row + 1,
                    col: coord.col,
                },
                Coord {
                    row: coord.row,
                    col: coord.col + 1,
                },
            ],
            //is the starting position of the animal;
            //there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
            'S' => {
                start_pos = Coord {
                    row: coord.row,
                    col: coord.col,
                };
                let mut start_nbrs = Vec::new();
                if coord.row > 0 {
                    if let Some(north) = input
                        .lines()
                        .nth((coord.row - 1).try_into().unwrap())
                        .unwrap()
                        .chars()
                        .nth(coord.col.try_into().unwrap())
                    {
                        if north == '|' || north == 'F' || north == '7' {
                            start_nbrs.push(Coord {
                                row: coord.row - 1,
                                col: coord.col,
                            });
                        }
                    }
                }
                if coord.row < (height - 1).try_into().unwrap() {
                    if let Some(south) = input
                        .lines()
                        .nth((coord.row + 1).try_into().unwrap())
                        .unwrap()
                        .chars()
                        .nth(coord.col.try_into().unwrap())
                    {
                        if south == '|' || south == 'J' || south == 'L' {
                            start_nbrs.push(Coord {
                                row: coord.row + 1,
                                col: coord.col,
                            });
                        }
                    }
                }
                if coord.col < (width - 1).try_into().unwrap() {
                    if let Some(east) = input
                        .lines()
                        .nth(coord.row.try_into().unwrap())
                        .unwrap()
                        .chars()
                        .nth((coord.col + 1).try_into().unwrap())
                    {
                        if east == '-' || east == 'J' || east == '7' {
                            start_nbrs.push(Coord {
                                row: coord.row,
                                col: coord.col + 1,
                            });
                        }
                    }
                }
                if coord.col > 0 {
                    if let Some(west) = input
                        .lines()
                        .nth(coord.row.try_into().unwrap())
                        .unwrap()
                        .chars()
                        .nth((coord.col - 1).try_into().unwrap())
                    {
                        if west == '-' || west == 'L' || west == 'F' {
                            start_nbrs.push(Coord {
                                row: coord.row,
                                col: coord.col - 1,
                            });
                        }
                    }
                }
                start_nbrs
            }
            _ => panic!("Unexpected symbol detected {}", symbol),
        };

        out_map.insert(hash_value, neighbours);
    });

    (start_pos, out_map)
}

#[derive(Debug, Copy, Clone)]
struct Coord {
    row: isize,
    col: isize,
}

impl Coord {
    fn hash(&self) -> usize {
        (self.row * 10000 + self.col).try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!(4, process(input));
    }

    #[test]
    fn test_process2() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(8, process(input));
    }
}
