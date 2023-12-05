use crate::part2::hash_coord;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
pub struct Schematic {
    grid: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
}

impl FromStr for Schematic {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<_>>();
        let height = grid.len();
        let width = grid.first().expect("Grid is non empty").len();

        Ok(Schematic {
            grid,
            height,
            width,
        })
    }
}

impl Schematic {
    pub fn get_cell(&self, row_i: usize, col_i: usize) -> Option<char> {
        let row = self.grid.get(row_i)?;
        let col = row.get(col_i)?;
        Some(*col)
    }

    fn get_min_max(&self, row: usize, col: usize) -> (usize, usize, usize, usize) {
        let row_min = if row == 0 { 0 } else { row - 1 };
        let row_max = if row == self.height - 1 {
            self.height
        } else {
            row + 2
        };
        let col_min = if col == 0 { 0 } else { col - 1 };
        let col_max = if col == self.width - 1 {
            self.width
        } else {
            col + 2
        };

        (row_min, row_max, col_min, col_max)
    }

    pub fn neighbours_symbol(&self, row: usize, col: usize) -> bool {
        let (row_min, row_max, col_min, col_max) = self.get_min_max(row, col);

        for i in row_min..row_max {
            for j in col_min..col_max {
                let cell = self.grid.get(i).unwrap().get(j).unwrap();
                if !(cell.is_ascii_digit() || *cell == '.') {
                    return true;
                }
            }
        }

        false
    }

    pub fn get_numbers(&self) -> Vec<Number> {
        let mut out: Vec<Number> = Vec::new();
        for i in 0..self.width {
            let row = self.grid.get(i).expect("i is in range");

            let mut current_number: Vec<u32> = Vec::new();
            let mut current_positions: Vec<(usize, usize)> = Vec::new();

            for j in 0..self.height {
                let cell = row.get(j).expect("j is in range");

                match cell.is_ascii_digit() {
                    true => {
                        current_number.push(cell.to_digit(10).expect("It's a number"));
                        current_positions.push((i, j));
                    }
                    false => {
                        if !current_number.is_empty() {
                            out.push(Number {
                                value: current_number.iter().fold(0, |acc, curr| acc * 10 + curr),
                                position: current_positions.clone(),
                            });
                            current_number = Vec::new();
                            current_positions = Vec::new();
                        }
                    }
                }
            }
            if !current_number.is_empty() {
                out.push(Number {
                    value: current_number.iter().fold(0, |acc, curr| acc * 10 + curr),
                    position: current_positions.clone(),
                });
            }
        }

        out
    }

    pub fn compute_gear(
        &self,
        row: usize,
        col: usize,
        position_map: &HashMap<usize, &Number>,
    ) -> u32 {
        let mut surrounding_numbers: Vec<u32> = Vec::new();

        let (row_min, row_max, col_min, col_max) = self.get_min_max(row, col);

        for i in row_min..row_max {
            for j in col_min..col_max {
                if let Some(cell) = self.get_cell(i, j) {
                    if cell.is_ascii_digit() {
                        if let Some(number) = position_map.get(&hash_coord(i, j)) {
                            if surrounding_numbers.contains(&number.value) {
                                continue;
                            }
                            surrounding_numbers.push(number.value);
                        }
                    }
                }
            }
        }

        if surrounding_numbers.len() == 2 {
            return surrounding_numbers
                .into_iter()
                .reduce(|acc, curr| acc * curr)
                .unwrap();
        }

        0
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Number {
    pub value: u32,
    pub position: Vec<(usize, usize)>,
}
