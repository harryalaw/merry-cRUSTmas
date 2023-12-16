use rayon::prelude::*;

#[tracing::instrument]
pub fn process(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let max_row = grid.len();
    let max_col = grid[0].len();

    if max_row != max_col {
        panic!("We're assuming that the row and columns are the same");
    }

    let positions: Vec<(usize, usize, Direction)> = (0..max_row)
        .flat_map(|idx| {
            vec![
                (0, idx, Direction::Down),
                (idx, 0, Direction::Right),
                (max_row - 1, idx, Direction::Up),
                (idx, max_row - 1, Direction::Left),
            ]
        })
        .collect();

    positions
        .into_par_iter()
        .map(|(row, col, dir)| {
            let mut to_visit = Vec::new();
            let pos = (row, col);
            let mirror = grid[row][col];
            let laser = Laser { pos, dir };
            reflect(laser, &mut to_visit, mirror);
            compute_energy(&grid, max_row, max_col, &mut to_visit)
        })
        .max()
        .expect("max value exists")
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
                let mirror = grid[new_laser.pos.0][new_laser.pos.1];
                reflect(new_laser, &mut next_iter, mirror);
            }
        }
        *to_visit = next_iter;
    }
}

fn reflect(laser: Laser, next_iter: &mut Vec<Laser>, mirror: char) {
    let dir = laser.dir;
    match (mirror, dir) {
        ('\\', Direction::Right) => next_iter.push(laser.with(Direction::Down)),
        ('\\', Direction::Down) => next_iter.push(laser.with(Direction::Right)),
        ('\\', Direction::Left) => next_iter.push(laser.with(Direction::Up)),
        ('\\', Direction::Up) => next_iter.push(laser.with(Direction::Left)),
        ('/', Direction::Right) => next_iter.push(laser.with(Direction::Up)),
        ('/', Direction::Up) => next_iter.push(laser.with(Direction::Right)),
        ('/', Direction::Down) => next_iter.push(laser.with(Direction::Left)),
        ('/', Direction::Left) => next_iter.push(laser.with(Direction::Down)),
        ('|', Direction::Down | Direction::Up) => next_iter.push(laser),
        ('|', Direction::Left | Direction::Right) => {
            next_iter.push(laser.with(Direction::Up));
            next_iter.push(laser.with(Direction::Down))
        }
        ('-', Direction::Left | Direction::Right) => next_iter.push(laser),
        ('-', Direction::Down | Direction::Up) => {
            next_iter.push(laser.with(Direction::Left));
            next_iter.push(laser.with(Direction::Right))
        }
        ('.', _) => next_iter.push(laser),
        (_, _) => panic!("Not a mirror"),
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
    fn with(&self, dir: Direction) -> Laser {
        Laser { pos: self.pos, dir }
    }

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
