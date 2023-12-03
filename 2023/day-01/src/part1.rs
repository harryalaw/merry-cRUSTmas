#[tracing::instrument]
pub fn process(input: &str) -> u32 {
    return input.lines().map(parse_words).sum();
}

fn parse_words(s: &str) -> u32 {
    let first_idx = s
        .find(|x: char| x.is_digit(10))
        .expect("should have one digit");
    let last_idx = s.rfind(|x: char| x.is_digit(10)).unwrap_or(first_idx);

    let first_number: u32 = s[first_idx..=first_idx]
        .parse()
        .expect("Failed to parse first digit");
    let last_number: u32 = s[last_idx..=last_idx]
        .parse()
        .expect("Failed to parse last digit");

    first_number * 10 + last_number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(142, process(input))
    }
}
