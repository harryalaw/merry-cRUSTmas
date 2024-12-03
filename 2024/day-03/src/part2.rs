use regex::Regex;

#[tracing::instrument]
pub fn process(input: &str) -> usize {
    let re = Regex::new(r"(mul\((\d+),(\d+)\)|do\(\)|don't\(\))").unwrap();

    let mut enabled = true;
    let mut sum = 0;

    for mat in re.captures_iter(input) {
        match &mat[0] {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                if enabled {
                    let num1 = &mat[2].parse::<usize>().unwrap();
                    let num2 = &mat[3].parse::<usize>().unwrap();
                    sum += num1 * num2;
                }
            }
        };
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(48, process(input));
    }
}
