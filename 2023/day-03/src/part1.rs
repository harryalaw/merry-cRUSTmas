use crate::schematic::{Schematic, Number};

pub fn process(input: &str) -> u32 {
    let schematic = input.parse::<Schematic>().unwrap();

    let numbers = schematic.get_numbers();

    numbers
        .iter()
        .filter(|number| is_part_number(&number, &schematic))
        .map(|number| {
            number.value
        })
        .sum()
}

fn is_part_number(number: &Number, schematic: &Schematic) -> bool {
    for (x, y) in number.position.iter() {
        if schematic.neighbours_symbol(*x, *y) {
            return true;
        }
    }

    false
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
        assert_eq!(4361, process(input));
    }
}
