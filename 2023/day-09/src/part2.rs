#[tracing::instrument]
pub fn process(input: &str) -> isize {
    input
        .lines()
        .map(parse_input)
        .map(|nums| prev_num(&nums))
        .sum()
}

fn parse_input(line: &str) -> Vec<isize> {
    line.split_ascii_whitespace()
        .flat_map(|x| x.parse::<isize>())
        .collect()
}

fn prev_num(nums: &[isize]) -> isize {
    if nums.iter().all(|x| x == &0) {
        return 0;
    }

    nums.first().expect("Has a value") - prev_num(&differences(nums))
}

fn differences(nums: &[isize]) -> Vec<isize> {
    let mut prev = nums.first().expect("Has a value");
    nums.iter()
        .skip(1)
        .map(|num| {
            let new = num - prev;
            prev = num;
            new
        })
        .collect()
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
