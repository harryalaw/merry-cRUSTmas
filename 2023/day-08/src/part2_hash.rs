use rayon::prelude::*;
use hashbrown::HashMap;
use std::hash::Hash;

#[tracing::instrument]
pub fn process(input: &str) -> usize {
    let (directions, map, initial_locations) = parse_input(input);

    initial_locations
        .par_iter()
        .map(|location| {
            let mut curr_location = *location;
            let mut steps = 0;
            while curr_location.value & 0xFF != 26 {
                let dir = directions
                    .get(steps % directions.len())
                    .expect("Should be in the range");

                let destinations = map.get(&curr_location).expect("Locations should be there");
                curr_location = match dir {
                    Dir::L => destinations.0,
                    Dir::R => destinations.1,
                };
                steps += 1;
            }
            steps
        })
        .reduce(|| 1, lcm)
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first.max(second);
    let mut min = first.min(second);

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

type ParseOutput = (
    Vec<Dir>,
    HashMap<Location, (Location, Location)>,
    Vec<Location>,
);

fn parse_input(input: &str) -> ParseOutput {
    let parts = input.split_once("\n\n").expect("unix endings");
    let directions = parts.0.chars().map(Dir::new).collect();

    let mut outmap = HashMap::new();
    let mut initial_locations = Vec::new();

    parts.1.lines().for_each(|line| {
        let halves = line.split_once(" = ").expect("It has an equals");
        let source = Location::new(halves.0);
        let left = Location::new(&halves.1[1..4]);
        let right = Location::new(&halves.1[6..9]);

        outmap.insert(source, (left, right));

        if source.value & 0xff == 1 {
            initial_locations.push(source);
        }
    });

    (directions, outmap, initial_locations)
}

#[derive(Eq, PartialEq, Clone, Copy, Hash)]
struct Location {
    value: usize,
}

impl Location {
    fn new(s: &str) -> Location {
        if s.len() != 3 {
            panic!("Should be length 3, {}", s.len());
        }
        let a = char_to_digit(s.chars().nth(0).expect("Exists"));
        let b = char_to_digit(s.chars().nth(1).expect("Exists"));
        let c = char_to_digit(s.chars().nth(2).expect("Exists"));

        Location {
            value: a << 16 | b << 8 | c,
        }
    }
}

fn char_to_digit(c: char) -> usize {
    match c {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        'D' => 4,
        'E' => 5,
        'F' => 6,
        'G' => 7,
        'H' => 8,
        'I' => 9,
        'J' => 10,
        'K' => 11,
        'L' => 12,
        'M' => 13,
        'N' => 14,
        'O' => 15,
        'P' => 16,
        'Q' => 17,
        'R' => 18,
        'S' => 19,
        'T' => 20,
        'U' => 21,
        'V' => 22,
        'W' => 23,
        'X' => 24,
        'Y' => 25,
        'Z' => 26,
        '1' => 27,
        '2' => 28,
        _ => 0,
    }
}

#[derive(Debug)]
enum Dir {
    R,
    L,
}

impl Dir {
    fn new(c: char) -> Dir {
        match c {
            'R' => Dir::R,
            'L' => Dir::L,
            _ => panic!("Not a valid char {}", c),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(6, process(input));
    }
}
