#[tracing::instrument]
pub fn process(input: &str) -> u32 {
    return input.lines().map(parse_words).sum();
}

fn parse_words(line: &str) -> u32 {
    let first = first_number(line).expect("should be a number");

    let last = last_number(line).unwrap_or(first);

    return first * 10 + last;
}

fn first_number(line: &str) -> Option<u32> {
    for i in 0..line.len() {
        if let Some(number) = forward_parse(line, i) {
            return Some(number);
        }
    }
    return None;
}

fn last_number(line: &str) -> Option<u32> {
    for i in (0..line.len()+1).rev() {
        if let Some(number) = reverse_parse(line, i) {
            return Some(number);
        }
    }
    return None;
}

fn forward_parse(line: &str, idx: usize) -> Option<u32> {
    let reduced = &line[idx..];

    if reduced.starts_with("one") {
        Some(1)
    } else if reduced.starts_with("two") {
        Some(2)
    } else if reduced.starts_with("three") {
        Some(3)
    } else if reduced.starts_with("four") {
        Some(4)
    } else if reduced.starts_with("five") {
        Some(5)
    } else if reduced.starts_with("six") {
        Some(6)
    } else if reduced.starts_with("seven") {
        Some(7)
    } else if reduced.starts_with("eight") {
        Some(8)
    } else if reduced.starts_with("nine") {
        Some(9)
    } else {
        reduced.chars().next().unwrap().to_digit(10)
    }
}

fn reverse_parse(line: &str, idx: usize) -> Option<u32> {
    let reduced = &line[..idx];

    if reduced.ends_with("one") {
        Some(1)
    } else if reduced.ends_with("two") {
        Some(2)
    } else if reduced.ends_with("three") {
        Some(3)
    } else if reduced.ends_with("four") {
        Some(4)
    } else if reduced.ends_with("five") {
        Some(5)
    } else if reduced.ends_with("six") {
        Some(6)
    } else if reduced.ends_with("seven") {
        Some(7)
    } else if reduced.ends_with("eight") {
        Some(8)
    } else if reduced.ends_with("nine") {
        Some(9)
    } else {
        reduced.chars().last().unwrap().to_digit(10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(281, process(input))
    }
}
