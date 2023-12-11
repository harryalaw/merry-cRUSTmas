use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[tracing::instrument]
pub fn process(input: &str) -> usize {
    let (directions, map) = parse_input(input);

    let mut steps = 0;
    let mut location = Location {
        a: 'A',
        b: 'A',
        c: 'A',
    };
    let target = Location {
        a: 'Z',
        b: 'Z',
        c: 'Z',
    };

    while location != target {
        let dir = directions
            .get(steps % directions.len())
            .expect("Should be in the range");
        let next_locations = map.get(&location).expect("Locations should be there");
        location = match dir {
            Dir::R => next_locations.1,
            Dir::L => next_locations.0,
        };
        steps += 1
    }

    steps
}

fn parse_input(input: &str) -> (Vec<Dir>, HashMap<Location, (Location, Location)>) {
    let parts = input.split_once("\n\n").expect("unix endings");
    let directions = parts.0.chars().map(Dir::new).collect();

    let mut outmap = HashMap::new();

    parts.1.lines().for_each(|line| {
        let halves = line.split_once(" = ").expect("It has an equals");
        let source = Location::new(halves.0);
        let left = Location::new(&halves.1[1..4]);
        let right = Location::new(&halves.1[6..9]);

        outmap.insert(source, (left, right));
    });

    (directions, outmap)
}

#[derive(Eq, PartialEq, Clone, Copy)]
struct Location {
    a: char,
    b: char,
    c: char,
}

impl Hash for Location {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.a.hash(state);
        self.b.hash(state);
        self.c.hash(state);
    }
}

impl Location {
    fn new(s: &str) -> Location {
        if s.len() != 3 {
            panic!("Should be length 3, {}", s.len());
        }
        Location {
            a: s.chars().nth(0).expect("Exists"),
            b: s.chars().nth(1).expect("Exists"),
            c: s.chars().nth(2).expect("Exists"),
        }
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
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(2, process(input));
    }

    #[test]
    fn test_process_2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(6, process(input));
    }
}
