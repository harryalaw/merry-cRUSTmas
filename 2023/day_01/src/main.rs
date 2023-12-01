use std::str::FromStr;

fn main() {
    do_part1()
}

fn do_part1() {
    use std::time::Instant;
    let now = Instant::now();
    println!("Part 1: {}", part1(include_str!("../input.txt")));
    let elapsed = now.elapsed();
    println!("Took: {:.2?}", elapsed);

    let now = Instant::now();
    println!("Part 2: {}", part2(include_str!("../input.txt")));
    let elapsed = now.elapsed();
    println!("Took: {:.2?}", elapsed);
}

fn part1(input: &str) -> i32 {
    return input
        .lines()
        .map(|line| line.parse::<Digits>().unwrap().value)
        .sum();
}

fn part2(input: &str) -> i32 {
    return input
        .lines()
        .map(|line| line.parse::<Words>().unwrap().value)
        .sum();
}

struct Digits {
    value: i32,
}

struct Words {
    value: i32,
}

const NUMBERS: &'static [&'static str] = &["1", "2", "3", "4", "5", "6", "7", "8", "9"];

const WORDS: &'static [&'static str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
    "5", "6", "7", "8", "9",
];

impl FromStr for Digits {
    type Err = ();

    fn from_str(s: &str) -> Result<Digits, Self::Err> {
        let value = parse_words(s, NUMBERS);
        return Ok(Digits { value });
    }
}

impl FromStr for Words {
    type Err = ();
    fn from_str(s: &str) -> Result<Words, Self::Err> {
        let value = parse_words(s, WORDS);

        return Ok(Words { value });
    }
}

struct Numbers<'a> {
    first: (usize, &'a str),
    last: (usize, &'a str),
}

impl Numbers<'_> {
    fn value(&self) -> i32 {
        let first_number = to_number(self.first.1);
        let last_number = to_number(self.last.1);

        first_number * 10 + last_number
    }
}

fn parse_words(s: &str, valid_words: &[&'static str]) -> i32 {
    valid_words
        .iter()
        .flat_map(|word| (s.match_indices(*word).into_iter()))
        .fold(
            Numbers {
                first: (usize::MAX, ""),
                last: (0, ""),
            },
            |acc, x| match (x.0 < acc.first.0, x.0 < acc.last.0) {
                (true, true) => Numbers {
                    first: x,
                    last: acc.last,
                },
                (true, false) => Numbers { first: x, last: x },
                (false, true) => Numbers {
                    first: acc.first,
                    last: acc.last,
                },
                (false, false) => Numbers {
                    first: acc.first,
                    last: x,
                },
            },
        )
        .value()
}

fn to_number(s: &str) -> i32 {
    match s {
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => 0,
    }
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

        assert_eq!(142, part1(input))
    }

    #[test]
    fn test_part2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(281, part2(input))
    }
}
