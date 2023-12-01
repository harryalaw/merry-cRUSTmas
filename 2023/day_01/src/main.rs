use std::str::FromStr;

fn main() {
    do_part1()
}

fn do_part1() {
    println!("Part 1: {}", part1(include_str!("../input.txt")));
    println!("Part 2: {}", part2(include_str!("../input.txt")));
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

enum Direction {
    FIRST,
    LAST,
}

fn find_number(s: &str, direction: Direction) -> char {
    let numbers: Vec<char> = s.chars().filter(|x| x.is_numeric()).collect();

    match direction {
        Direction::FIRST => *numbers.first().expect("We should have numbers"),
        Direction::LAST => *numbers.last().expect("We should have numbers"),
    }
}

struct Digits {
    value: i32,
}

struct Words {
    value: i32,
}

impl FromStr for Digits {
    type Err = ();

    fn from_str(s: &str) -> Result<Digits, Self::Err> {
        let first_number = find_number(s, Direction::FIRST);
        let last_number = find_number(s, Direction::LAST);
        let value = format!("{}{}", first_number, last_number);
        return Ok(Digits {
            value: value.parse::<i32>().expect("Value should be numbers"),
        });
    }
}

fn parse_words(s: &str) -> i32 {
    let words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];

    let mut first_positions: Vec<(usize, &str)> = words
        .iter()
        .flat_map(|word| s.match_indices(*word).next())
        .collect();

    first_positions.sort_by(|a, b| a.0.cmp(&b.0));

    let mut last_positions: Vec<(usize, &str)> = words
        .iter()
        .flat_map(|word| s.rmatch_indices(*word).next())
        .collect();
    last_positions.sort_by(|a, b| b.0.cmp(&a.0));

    dbg!(s, first_positions.first(), last_positions.first());
    let first_number = *(first_positions.first().unwrap());
    let last_number = *(last_positions.first().unwrap());

    format!(
        "{}{}",
        to_number(first_number.1),
        to_number(last_number.1)
    )
    .parse::<i32>()
    .expect("It's all numbers")
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

impl FromStr for Words {
    type Err = ();
    fn from_str(s: &str) -> Result<Words, Self::Err> {
        let value = parse_words(s);

        return Ok(Words { value });
    }
}
