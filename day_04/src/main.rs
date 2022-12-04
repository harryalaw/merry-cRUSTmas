use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    do_part1();
    println!();
    do_part2();
}

fn do_part1() {
    let test_input = std::fs::read_to_string("./test.txt").unwrap();
    println!("Test 1: {}", part1(test_input));
    let real_input = std::fs::read_to_string("./input.txt").unwrap();
    println!("Part 1: {}", part1(real_input));
}

fn do_part2() {
    let test_input = std::fs::read_to_string("./test.txt").unwrap();
    println!("Test 2: {}", part2(test_input));
    let real_input = std::fs::read_to_string("./input.txt").unwrap();
    println!("Part 2: {}", part2(real_input));
}

struct Tasks {
    left: Task,
    right: Task,
}

struct Task {
    start: usize,
    end: usize,
}

impl FromStr for Tasks {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(',').expect("AOC Format for line");
        return Ok(Tasks {
            left: left.parse()?,
            right: right.parse()?,
        });
    }
}

impl FromStr for Task {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').expect("AOC Format for range");
        return Ok(Task {
            start: start.parse()?,
            end: end.parse()?,
        });
    }
}

impl Tasks {
    fn contains(&self) -> bool {
        return self.left.start <= self.right.start && self.right.end <= self.left.end
            || self.right.start <= self.left.start && self.left.end <= self.right.end;
    }

    fn disjoint(&self) -> bool {
        return self.left.end < self.right.start || self.right.end < self.left.start;
    }
}

fn part1(input: String) -> usize {
    return input
        .lines()
        .map(|line| line.parse::<Tasks>().unwrap())
        .filter(Tasks::contains)
        .count();
}

fn part2(input: String) -> usize {
    return input
        .lines()
        .map(|line| line.parse::<Tasks>().unwrap())
        .filter(|pair| !Tasks::disjoint(pair))
        .count();
}
