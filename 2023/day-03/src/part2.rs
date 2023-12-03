use crate::schematic::{Number, Schematic};
use std::collections::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> u32 {
    let schematic = input.parse::<Schematic>().unwrap();

    let numbers = schematic.get_numbers();
    let position_map = build_map(&numbers);

    let mut total = 0;
    for row in 0..schematic.height {
        for col in 0..schematic.width {
            if let Some(cell) = schematic.get_cell(row, col) {
                if cell == '*' {
                    total = schematic.compute_gear(row, col, &position_map) + total
                }
            }
        }
    }

    total
}

pub fn hash_coord(row: usize, col: usize) -> usize {
    row + 1000 * col
}

fn build_map(numbers: &Vec<Number>) -> HashMap<usize, &Number> {
    let mut out = HashMap::new();

    for number in numbers {
        for position in &number.position {
            out.insert(hash_coord(position.0, position.1), number);
        }
    }

    return out;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(467835, process(input));
    }
}
