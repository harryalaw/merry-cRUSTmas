use std::collections::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> usize {
    let (directions, map, initial_locations) = parse_input(input);

    let mut steps_counts: Vec<usize> = Vec::new();
    for location in initial_locations {
        let mut curr_location = location;
        let mut steps = 0;
        while !curr_location.ends_with('Z') {
            let dir = directions
                .get(steps % directions.len())
                .expect("Should be in the range");

            let destinations = map.get(curr_location).expect("Locations should be there");
            let next_location = match dir {
                Dir::L => destinations.0,
                Dir::R => destinations.1,
            };
            steps += 1;
            curr_location = next_location;
        }
        steps_counts.push(steps);
    }

    steps_counts.iter().fold(1, |total, curr| lcm(total, *curr))
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

type ParseOutput<'a> = (Vec<Dir>, HashMap<&'a str, (&'a str, &'a str)>, Vec<&'a str>);

fn parse_input(input: &str) -> ParseOutput {
    let parts = input.split_once("\n\n").expect("unix endings");
    let directions = parts.0.chars().map(Dir::new).collect();

    let mut outmap = HashMap::new();
    let mut initial_locations = Vec::new();

    parts.1.lines().for_each(|line| {
        let halves = line.split_once(" = ").expect("It has an equals");
        let source = halves.0;
        let left = &halves.1[1..4];
        let right = &halves.1[6..9];

        outmap.insert(source, (left, right));

        if source.ends_with('A') {
            initial_locations.push(source);
        }
    });

    (directions, outmap, initial_locations)
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
