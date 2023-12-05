use std::str::FromStr;

#[tracing::instrument]
pub fn process(input: &str) -> usize {
    let parts = input.split_once("\n\n").expect("Unix endings");
    let seeds = parse_seeds(parts.0);
    dbg!(&seeds);
    let mappings: Vec<Vec<Mapping>> = parts.1.split("\n\n").map(parse_maps).collect();

    *(apply_mappings(seeds, &mappings)
        .iter()
        .min()
        .expect("It's a number"))
}

fn parse_seeds(s: &str) -> Vec<Interval> {
    let seeds = s.split_once(": ").unwrap().1.split(' ');
    let mut starts = Vec::new();
    let mut ranges = Vec::new();
    seeds.enumerate().for_each(|(i, val)| match i % 2 == 0 {
        true => starts.push(val.parse::<usize>().expect("number")),
        false => ranges.push(val.parse::<usize>().expect("number")),
    });
    starts
        .iter()
        .zip(ranges.iter())
        .map(|(start, range)| Interval::new(*start, start + range))
        .collect()
}

fn parse_maps(s: &str) -> Vec<Mapping> {
    s.lines()
        .skip(1)
        .flat_map(|line| line.parse::<Mapping>())
        .collect()
}

fn apply_mappings(intervals: Vec<Interval>, mappings: &[Vec<Mapping>]) -> Vec<usize> {
    let mut values: Vec<usize> = Vec::new();
    for seed in &intervals {
        for i in seed.values() {
            values.push(i);
        }
    }

    for (i, mapping) in mappings.iter().enumerate() {
        // let new_intervals = compute_intervals(mapping, &intervals);
        values = map_ranges(mapping, &values);
        println!("Done mapping {}", i);
        println!("Min so far {}", values.iter().min().unwrap());
    }

    values
}

fn map_ranges(mappings: &[Mapping], values: &Vec<usize>) -> Vec<usize> {
    let mut out = Vec::new();
    for val in values {
        out.push(map_number(mappings, *val));
    }
    out
}


#[derive(Eq, PartialEq, Debug, Clone)]
struct Interval {
    start: usize,
    // end is exclusive
    end: usize,
}

impl Interval {
    fn new(start: usize, end: usize) -> Interval {
        Interval { start, end }
    }

    fn values(&self) -> Vec<usize> {
        (self.start..self.end).collect()
    }

    fn contains(&self, x: usize) -> bool {
        self.start <= x && x < self.end
    }
}


#[derive(Debug)]
struct Mapping {
    dest_start: usize,
    source_start: usize,
    source_interval: Interval,
}

impl FromStr for Mapping {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = s.split_ascii_whitespace();
        let dest_start = numbers
            .next()
            .unwrap()
            .parse::<usize>()
            .expect("It's a number");
        let source_start = numbers
            .next()
            .unwrap()
            .parse::<usize>()
            .expect("It's a number");
        let range = numbers
            .next()
            .unwrap()
            .parse::<usize>()
            .expect("It's a number");
        Ok(Mapping {
            dest_start,
            source_start,
            source_interval: Interval::new(source_start, source_start + range),
        })
    }
}

fn map_number(mappings: &[Mapping], x: usize) -> usize {
    for mapping in mappings.iter() {
        if mapping.source_interval.contains(x) {
            let offset = x - mapping.source_start;
            return mapping.dest_start + offset;
        }
    }

    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_number() {
        let mut mappings: Vec<Mapping> = Vec::new();
        mappings.push(Mapping {
            dest_start: 50,
            source_start: 98,
            source_interval: Interval::new(98, 100),
        });
        mappings.push(Mapping {
            dest_start: 52,
            source_start: 50,
            source_interval: Interval::new(50, 98),
        });

        assert_eq!(81, map_number(&mappings, 79));
        assert_eq!(14, map_number(&mappings, 14));
        assert_eq!(57, map_number(&mappings, 55));
        assert_eq!(13, map_number(&mappings, 13));
        assert_eq!(0, map_number(&mappings, 0));
        assert_eq!(1, map_number(&mappings, 1));
        assert_eq!(48, map_number(&mappings, 48));
        assert_eq!(49, map_number(&mappings, 49));
        assert_eq!(52, map_number(&mappings, 50));
        assert_eq!(53, map_number(&mappings, 51));
        assert_eq!(99, map_number(&mappings, 97));
        assert_eq!(50, map_number(&mappings, 98));
        assert_eq!(51, map_number(&mappings, 99));
    }

    #[test]
    fn test_process() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(46, process(input));
    }
}
