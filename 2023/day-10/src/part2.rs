use std::collections::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> usize {
    let (start, map) = parse_input(input);
    let visited = traverse(map, start);

    let height = input.lines().count();

    input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            if row == 0 || row == height - 1 {
                return 0;
            }
            let mut inside_count = 0;
            let mut pipe_count = 0;

            line.chars().enumerate().for_each(|(col, char)| {
                let coord = Coord {
                    row: row.try_into().unwrap(),
                    col: col.try_into().unwrap(),
                };
                match visited[u_size(coord.row)][u_size(coord.col)] {
                    true => match char {
                        '|' | 'L' | 'J' => pipe_count += 1,
                        _ => {}
                    },
                    false => inside_count += pipe_count % 2,
                }
            });
            inside_count
        })
        .sum()
}

fn traverse(pipe_grid: PipeGrid, start: Coord) -> Vec<Vec<bool>> {
    let mut visited = vec![vec![false; pipe_grid.width]; pipe_grid.height];

    let mut curr_coords = vec![start];
    visited[u_size(start.row)][u_size(start.col)] = true;

    while !curr_coords.is_empty() {
        let mut next_coords = Vec::new();

        for coord in curr_coords {
            let nbrs = pipe_grid
                .pipe_map
                .get(&coord.hash())
                .expect("Should be in map");
            for nbr in nbrs {
                if !visited[u_size(nbr.row)][u_size(nbr.col)] {
                    next_coords.push(*nbr);
                    visited[u_size(nbr.row)][u_size(nbr.col)] = true;
                }
            }
        }

        curr_coords = next_coords
    }

    visited
}

fn u_size(val: isize) -> usize {
    val.try_into().unwrap()
}

struct PipeGrid {
    width: usize,
    height: usize,
    pipe_map: HashMap<usize, Vec<Coord>>,
}

fn parse_input(input: &str) -> (Coord, PipeGrid) {
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

    let mut pipe_map = HashMap::new();

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

        pipe_map.insert(hash_value, neighbours);
    });

    (
        start_pos,
        PipeGrid {
            width,
            height,
            pipe_map,
        },
    )
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
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!(4, process(input));
    }

    #[test]
    fn test_process2() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(8, process(input));
    }

    #[test]
    fn test_process3() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(10, process(input));
    }
}
