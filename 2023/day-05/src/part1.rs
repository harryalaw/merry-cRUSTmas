use std::str::FromStr;

#[tracing::instrument]
pub fn process(input: &str) -> usize {
    let parts = input.split_once("\n\n").expect("Unix endings");
    let seeds = parse_seeds(parts.0);
    let mappings: Vec<Vec<Mapping>> = parts.1.split("\n\n").map(parse_maps).collect();

    seeds
        .iter()
        .map(|seed| apply_mappings(*seed, &mappings))
        .min()
        .expect("Should be a min value")
}

fn parse_seeds(s: &str) -> Vec<usize> {
    s.split_once(": ")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .flat_map(|x| x.parse::<usize>())
        .collect()
}

fn parse_maps(s: &str) -> Vec<Mapping> {
    s.lines()
        .skip(1)
        .flat_map(|line| line.parse::<Mapping>())
        .collect()
}

fn apply_mappings(x: usize, mappings: &Vec<Vec<Mapping>>) -> usize {
    let mut val = x;
    for mapping in mappings {
        val = map_number(mapping, val);
    }
    val
}

#[derive(Debug)]
struct Mapping {
    dest_start: usize,
    source_start: usize,
    range: usize,
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
            range,
        })
    }
}

fn map_number(mappings: &[Mapping], x: usize) -> usize {
    for mapping in mappings.iter() {
        if mapping.source_start <= x && x <= mapping.source_start + mapping.range {
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
            range: 2,
        });
        mappings.push(Mapping {
            dest_start: 52,
            source_start: 50,
            range: 48,
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
        assert_eq!(35, process(input));
    }
}
