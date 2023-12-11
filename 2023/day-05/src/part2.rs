use rayon::prelude::*;
use std::str::FromStr;

#[tracing::instrument]
pub fn process(input: &str) -> usize {
    let parts = input.split_once("\n\n").expect("Unix endings");
    let seeds = parse_seeds(parts.0);
    let mappings: Vec<Vec<Mapping>> = parts.1.split("\n\n").map(parse_maps).collect();

    let split_seeds: Vec<Vec<Interval>> = seeds.iter().map(|seed| vec![seed.clone()]).collect();

    split_seeds
        .par_iter()
        .flat_map(|seed| compute_final_intervals(seed, &mappings))
        .map(|i| i.start)
        .min()
        .expect("It's a number")
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

fn compute_final_intervals(intervals: &[Interval], mappings: &Vec<Vec<Mapping>>) -> Vec<Interval> {
    let mut new_intervals = intervals.to_owned();

    for mapping in mappings {
        new_intervals = apply_mappings(mapping, &new_intervals);
    }

    new_intervals
}

fn apply_mappings(mappings: &Vec<Mapping>, intervals: &[Interval]) -> Vec<Interval> {
    // for each mapping we need to get the list of intersections and non intersections
    let mut curr_intervals = intervals.to_owned();
    let mut mapped_intervals = Vec::new();

    for mapping in mappings {
        let mut next_intervals: Vec<Interval> = Vec::new();
        for interval in curr_intervals {
            let (intersection, untouched) = interval.overlaps(&mapping.source_interval);
            if let Some(mapped) = intersection {
                mapped_intervals.push(map_interval(mapped, mapping));
                untouched.iter().for_each(|leftover| {
                    next_intervals.push(leftover.clone());
                });
            } else {
                next_intervals.push(interval.clone());
            }
        }
        curr_intervals = next_intervals.into_iter().collect();
    }

    // these are unmapped so should stay as they are:
    curr_intervals
        .iter()
        .for_each(|slice| mapped_intervals.push(slice.clone()));

    mapped_intervals
}

fn map_interval(interval: Interval, mapping: &Mapping) -> Interval {
    let start_offset = interval.start - mapping.source_start;
    let end_offset = interval.end - mapping.source_start;
    let new_start = mapping.dest_start + start_offset;
    let new_end = mapping.dest_start + end_offset;
    Interval::new(new_start, new_end)
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
struct Interval {
    start: usize,
    // end is exclusive
    end: usize,
}

impl Interval {
    fn new(start: usize, end: usize) -> Interval {
        Interval { start, end }
    }

    fn intersect(&self, other: &Interval) -> Option<Interval> {
        // a b c d
        // c d a b
        if self.end <= other.start || other.end <= self.start {
            None
        } else if self.start <= other.start && other.start < self.end && self.end <= other.end {
            // a c b d
            Some(Interval::new(other.start, self.end))
        } else if self.start <= other.start && other.end <= self.end {
            // a c d b
            Some(Interval::new(other.start, other.end))
        } else if other.start <= self.start && self.start < other.end && other.end <= self.end {
            // c a d b
            Some(Interval::new(self.start, other.end))
        } else {
            // c a b d
            Some(Interval::new(self.start, self.end))
        }
    }

    fn overlaps(&self, other: &Interval) -> (Option<Interval>, Vec<Interval>) {
        let intersection = self.intersect(other);

        if intersection.is_none() {
            return (None, vec![self.clone()]);
        }
        let it = intersection.unwrap();

        let mut remaining = Vec::new();

        // 3: a c d b => [a,c), [b,d)
        // 2: a c b d => [a,c)
        // 5: c a d b => [d,b)
        // 6: c a b d =>
        match (self.start < it.start, it.end < self.end) {
            (true, true) => {
                remaining.push(Interval::new(self.start, it.start));
                remaining.push(Interval::new(it.end, self.end));
            }
            (true, false) => {
                remaining.push(Interval::new(self.start, it.start));
            }
            (false, true) => {
                remaining.push(Interval::new(it.end, self.end));
            }
            (false, false) => {}
        }

        (Some(it), remaining)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlaps_disjoint() {
        let first = Interval::new(1, 3);
        let second = Interval::new(3, 4);

        let disjoint_1 = first.overlaps(&second);

        assert_eq!(None, disjoint_1.0);
        assert_eq!(vec![first], disjoint_1.1);
    }

    #[test]
    fn test_overlaps_intersection_first_end() {
        let first = Interval::new(1, 4);
        let second = Interval::new(3, 5);

        let disjoint_1 = first.overlaps(&second);
        let disjoint_2 = second.overlaps(&first);

        let expected = vec![Interval::new(1, 3)];
        assert_eq!(disjoint_1.0, disjoint_2.0);
        assert_eq!(Some(Interval::new(3, 4)), disjoint_1.0);
        assert_eq!(expected, disjoint_1.1);
    }

    #[test]
    fn test_overlaps_intersection_last() {
        let first = Interval::new(2, 4);
        let second = Interval::new(1, 3);

        let disjoint_1 = first.overlaps(&second);

        let expected: Vec<Interval> = vec![Interval::new(3, 4)];
        assert_eq!(Some(Interval::new(2, 3)), disjoint_1.0);
        assert_eq!(expected, disjoint_1.1);
    }

    #[test]
    fn test_overlaps_contains() {
        let first = Interval::new(2, 4);
        let second = Interval::new(1, 5);

        let overlap_1 = first.overlaps(&second);
        let overlap_2 = second.overlaps(&first);

        let expected: Vec<Interval> = Vec::new();
        assert_eq!(overlap_1.0, overlap_2.0);
        assert_eq!(Some(Interval::new(2, 4)), overlap_1.0);
        assert_eq!(expected, overlap_1.1);
    }

    #[test]
    fn test_intersections_disjoint() {
        let first = Interval::new(1, 3);
        let second = Interval::new(3, 4);

        let disjoint_1 = first.intersect(&second);
        let disjoint_2 = second.intersect(&first);

        assert_eq!(disjoint_1, disjoint_2);
        assert_eq!(None, disjoint_1);
    }

    #[test]
    fn test_intersections_overlaps() {
        let first = Interval::new(1, 4);
        let second = Interval::new(3, 5);

        let overlap_1 = first.intersect(&second).unwrap();
        // 1 3 4 5
        let overlap_2 = second.intersect(&first).unwrap();

        let expected = Interval::new(3, 4);

        assert_eq!(overlap_1, overlap_2);
        assert_eq!(expected, overlap_1);
        assert_eq!(expected, overlap_2);
    }

    #[test]
    fn test_intersections_contains() {
        let first = Interval::new(1, 5);
        let second = Interval::new(3, 4);

        let contains_1 = first.intersect(&second).unwrap();
        let contains_2 = second.intersect(&first).unwrap();

        assert_eq!(contains_1, contains_2);
        assert_eq!(second, contains_1);
        assert_eq!(second, contains_2);
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

// 69323689 is too high :(
// So something in our brute force ain't working :(
// manual takes like 3 minutes :D
