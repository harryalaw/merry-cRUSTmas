#[tracing::instrument]
pub fn process(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let max_row = grid.len();
    let max_col = grid[0].len();

    let mut max_energy = 0;

    for row in 0..max_row {
        let mut to_visit = Vec::new();
        let pos = (row, 0);
        match grid[row][0] {
            '\\' => {
                to_visit.push(Laser {
                    pos,
                    dir: Direction::Down,
                });
            }
            '/' => {
                to_visit.push(Laser {
                    pos,
                    dir: Direction::Up,
                });
            }
            '|' => {
                to_visit.push(Laser {
                    pos,
                    dir: Direction::Up,
                });
                to_visit.push(Laser {
                    pos,
                    dir: Direction::Down,
                });
            }
            _ => {
                to_visit.push(Laser {
                    pos,
                    dir: Direction::Right,
                });
            }
        }
        let energy = compute_energy(&grid, max_row, max_col, &mut to_visit);
        if energy > max_energy {
            max_energy = energy;
        }

        let mut to_visit = Vec::new();
        let pos = (row, max_col - 1);
        match grid[row][max_col - 1] {
            '\\' => {
                to_visit.push(Laser {
                    pos,
                    dir: Direction::Up,
                });
            }
            '/' => {
                to_visit.push(Laser {
                    pos,
                    dir: Direction::Down,
                });
            }
            '|' => {
                to_visit.push(Laser {
                    pos,
                    dir: Direction::Up,
                });
                to_visit.push(Laser {
                    pos,
                    dir: Direction::Down,
                });
            }
            _ => {
                to_visit.push(Laser {
                    pos,
                    dir: Direction::Left,
                });
            }
        }
        let energy = compute_energy(&grid, max_row, max_col, &mut to_visit);
        if energy > max_energy {
            max_energy = energy;
        }
    }

    for col in 0..max_col {
        let mut to_visit = Vec::new();
        let pos = (0, col);
        match grid[0][col] {
            '\\' => {
                to_visit.push(Laser {
                    pos,
                    dir: Direction::Right,
                });
            }
            '/' => {
                to_visit.push(Laser {
                    pos,
                    dir: Direction::Left,
                });
            }
            '-' => {
                to_visit.push(Laser {
                    pos,
                    dir: Direction::Right,
                });
                to_visit.push(Laser {
                    pos,
                    dir: Direction::Left,
                });
            }
            _ => {
                to_visit.push(Laser {
                    pos,
                    dir: Direction::Down,
                });
            }
        }
        let energy = compute_energy(&grid, max_row, max_col, &mut to_visit);
        if energy > max_energy {
            max_energy = energy;
        }

        let mut to_visit = Vec::new();
        let pos = (max_row - 1, col);
        match grid[max_row - 1][col] {
            '\\' => {
                to_visit.push(Laser {
                    pos,
                    dir: Direction::Left,
                });
            }
            '/' => {
                to_visit.push(Laser {
                    pos,
                    dir: Direction::Right,
                });
            }
            '-' => {
                to_visit.push(Laser {
                    pos,
                    dir: Direction::Right,
                });
                to_visit.push(Laser {
                    pos,
                    dir: Direction::Left,
                });
            }
            _ => {
                to_visit.push(Laser {
                    pos,
                    dir: Direction::Up,
                });
            }
        }
        let energy = compute_energy(&grid, max_row, max_col, &mut to_visit);
        if energy > max_energy {
            max_energy = energy;
        }
    }

    max_energy
}

fn compute_energy(
    grid: &[Vec<char>],
    max_row: usize,
    max_col: usize,
    to_visit: &mut Vec<Laser>,
) -> usize {
    let mut energies: Vec<Vec<(bool, bool, bool, bool)>> = grid
        .iter()
        .map(|line| vec![(false, false, false, false); line.len()])
        .collect();

    let initial_lasers = to_visit.clone();

    do_visit(&mut energies, to_visit, max_row, max_col, grid);

    for laser in initial_lasers.iter() {
        update_energies(&mut energies, laser);
    }

    energies
        .iter()
        .map(|line| line.iter().filter(|x| x.0 || x.1 || x.2 || x.3).count())
        .sum()
}

fn do_visit(
    energies: &mut [Vec<(bool, bool, bool, bool)>],
    to_visit: &mut Vec<Laser>,
    max_row: usize,
    max_col: usize,
    grid: &[Vec<char>],
) {
    while !to_visit.is_empty() {
        let mut next_iter = Vec::new();
        for laser in to_visit.iter() {
            if seen_laser(energies, laser) {
                continue;
            }
            update_energies(energies, laser);

            if let Some(new_laser) = laser.travel(max_row, max_col) {
                match grid[new_laser.pos.0][new_laser.pos.1] {
                    '.' => {
                        next_iter.push(new_laser);
                    }
                    '/' => match new_laser.dir {
                        Direction::Up => {
                            let (row, col) = new_laser.pos;
                            if col != max_col {
                                next_iter.push(Laser {
                                    pos: (row, col),
                                    dir: Direction::Right,
                                });
                            }
                        }
                        Direction::Left => {
                            let (row, col) = new_laser.pos;
                            if row != max_row {
                                next_iter.push(Laser {
                                    pos: (row, col),
                                    dir: Direction::Down,
                                });
                            }
                        }
                        Direction::Down => {
                            let (row, col) = new_laser.pos;
                            if row != 0 {
                                next_iter.push(Laser {
                                    pos: (row, col),
                                    dir: Direction::Left,
                                });
                            }
                        }
                        Direction::Right => {
                            let (row, col) = new_laser.pos;
                            if col != 0 {
                                next_iter.push(Laser {
                                    pos: (row, col),
                                    dir: Direction::Up,
                                });
                            }
                        }
                    },
                    '\\' => match new_laser.dir {
                        Direction::Up => {
                            let (row, col) = new_laser.pos;
                            if col != 0 {
                                next_iter.push(Laser {
                                    pos: (row, col),
                                    dir: Direction::Left,
                                });
                            }
                        }
                        Direction::Left => {
                            let (row, col) = new_laser.pos;
                            if row != 0 {
                                next_iter.push(Laser {
                                    pos: (row, col),
                                    dir: Direction::Up,
                                });
                            }
                        }
                        Direction::Down => {
                            let (row, col) = new_laser.pos;
                            if col != max_col {
                                next_iter.push(Laser {
                                    pos: (row, col),
                                    dir: Direction::Right,
                                });
                            }
                        }
                        Direction::Right => {
                            let (row, col) = new_laser.pos;
                            if row != max_row {
                                next_iter.push(Laser {
                                    pos: (row, col),
                                    dir: Direction::Down,
                                });
                            }
                        }
                    },
                    '|' => match new_laser.dir {
                        Direction::Up | Direction::Down => {
                            next_iter.push(new_laser);
                        }
                        Direction::Left | Direction::Right => {
                            let (row, col) = new_laser.pos;
                            if row != 0 {
                                let next_laser = Laser {
                                    pos: (row, col),
                                    dir: Direction::Up,
                                };
                                next_iter.push(next_laser)
                            }
                            if row != max_row {
                                let next_laser = Laser {
                                    pos: (row, col),
                                    dir: Direction::Down,
                                };

                                next_iter.push(next_laser)
                            }
                        }
                    },
                    '-' => match new_laser.dir {
                        Direction::Left | Direction::Right => {
                            next_iter.push(new_laser);
                        }
                        Direction::Up | Direction::Down => {
                            let (row, col) = new_laser.pos;
                            if col != 0 {
                                let next_laser = Laser {
                                    pos: (row, col),
                                    dir: Direction::Left,
                                };

                                next_iter.push(next_laser)
                            }
                            if col != max_col {
                                let next_laser = Laser {
                                    pos: (row, col),
                                    dir: Direction::Right,
                                };

                                next_iter.push(next_laser)
                            }
                        }
                    },
                    _ => panic!(
                        "Not another one! {}",
                        grid[new_laser.pos.0][new_laser.pos.1]
                    ),
                }
            }
        }

        *to_visit = next_iter;
    }
}

fn update_energies(energies: &mut [Vec<(bool, bool, bool, bool)>], laser: &Laser) {
    let cell = &mut energies[laser.pos.0][laser.pos.1];
    match laser.dir {
        Direction::Up => cell.0 = true,
        Direction::Left => cell.1 = true,
        Direction::Down => cell.2 = true,
        Direction::Right => cell.3 = true,
    }
}

fn seen_laser(energies: &[Vec<(bool, bool, bool, bool)>], laser: &Laser) -> bool {
    let cell = energies[laser.pos.0][laser.pos.1];
    match laser.dir {
        Direction::Up => cell.0,
        Direction::Left => cell.1,
        Direction::Down => cell.2,
        Direction::Right => cell.3,
    }
}

#[derive(Copy, Clone, Debug)]
struct Laser {
    pos: (usize, usize),
    dir: Direction,
}

impl Laser {
    fn travel(&self, max_row: usize, max_col: usize) -> Option<Laser> {
        match self.dir {
            Direction::Up => {
                if self.pos.0 == 0 {
                    None
                } else {
                    Some(Laser {
                        dir: self.dir,
                        pos: (self.pos.0 - 1, self.pos.1),
                    })
                }
            }
            Direction::Left => {
                if self.pos.1 == 0 {
                    None
                } else {
                    Some(Laser {
                        dir: self.dir,
                        pos: (self.pos.0, self.pos.1 - 1),
                    })
                }
            }
            Direction::Down => {
                if self.pos.0 >= max_row - 1 {
                    None
                } else {
                    Some(Laser {
                        dir: self.dir,
                        pos: (self.pos.0 + 1, self.pos.1),
                    })
                }
            }
            Direction::Right => {
                if self.pos.1 >= max_col - 1 {
                    None
                } else {
                    Some(Laser {
                        dir: self.dir,
                        pos: (self.pos.0, self.pos.1 + 1),
                    })
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";
        assert_eq!(51, process(input));
    }
}
