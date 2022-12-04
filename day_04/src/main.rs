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

#[derive(Debug)]
struct Pair {
    first_start: usize,
    first_end: usize,
    second_start: usize,
    second_end: usize,
}

impl From<&str> for Pair {
    fn from(line: &str) -> Self {
        let parts: Vec<usize> = line
            .split(|c| c == ',' || c == '-')
            .map(|c| c.parse::<usize>().unwrap())
            .collect();
        return Pair {
            first_start: parts[0],
            first_end: parts[1],
            second_start: parts[2],
            second_end: parts[3],
        };
    }
}

impl Pair {
    fn contains(&self) -> bool {
        return self.first_start <= self.second_start && self.second_end <= self.first_end
            || self.second_start <= self.first_start && self.first_end <= self.second_end;
    }

    fn disjoint(&self) -> bool {
        return self.first_end < self.second_start || self.second_end < self.first_start;
    }
}

fn part1(input: String) -> usize {
    return input.lines().map(Pair::from).filter(Pair::contains).count();
}

fn part2(input: String) -> usize {
    return input
        .lines()
        .map(Pair::from)
        .filter(|pair| !Pair::disjoint(pair))
        .count();
}
