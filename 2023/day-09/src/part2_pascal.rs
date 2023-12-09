use crate::pascal::Pascal;

#[tracing::instrument]
pub fn process(input: &str) -> isize {
    let mut pascal = Pascal::new();
    input
        .lines()
        .map(parse_input)
        .map(|nums| get_value(&nums, &mut pascal))
        .sum()
}

fn parse_input(line: &str) -> Vec<isize> {
    line.split_ascii_whitespace()
        .flat_map(|x| x.parse::<isize>())
        .collect()
}

fn get_value(nums: &[isize], pascal: &mut Pascal) -> isize {
    let parity = nums.len() % 2;
    nums.iter()
        .rev()
        .enumerate()
        .map(|(i, val)| match i % 2 == parity {
            true => -pascal.choose(nums.len() + 1, i) * val,
            false => pascal.choose(nums.len() + 1, i) * val,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(2, process(input));
    }
}
