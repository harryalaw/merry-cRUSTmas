use regex::Regex;

#[tracing::instrument]
pub fn process(_input: &str) -> usize {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut sum = 0;

    for mat in re.captures_iter(_input) {
        let num1 = &mat[1].parse::<usize>().unwrap();
        let num2 = &mat[2].parse::<usize>().unwrap();
        sum += num1 * num2;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(161, process(input));
    }
}
