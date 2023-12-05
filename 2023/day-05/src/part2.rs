use std::{collections::HashSet, str::FromStr};

#[tracing::instrument]
pub fn process(input: &str) -> usize {
    let parts = input.split_once("\n\n").expect("Unix endings");
    let seeds = parse_seeds(parts.0);
    let mappings: Vec<Vec<Mapping>> = parts.1.split("\n\n").map(|map| parse_maps(map)).collect();

    let final_intervals = compute_final_intervals(seeds, &mappings);
    final_intervals
        .iter()
        .map(|i| i.start)
        .min()
        .expect("It's a number")
}

fn parse_seeds(s: &str) -> Vec<Interval> {
    let seeds = s.split_once(": ").unwrap().1.split(" ");
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

fn compute_final_intervals(
    intervals: Vec<Interval>,
    mappings: &Vec<Vec<Mapping>>,
) -> Vec<Interval> {
    let mut new_intervals = intervals.clone();

    for mapping in mappings {
        new_intervals = apply_mappings(mapping, &new_intervals);
    }

    new_intervals
}

fn apply_mappings(mappings: &Vec<Mapping>, intervals: &Vec<Interval>) -> Vec<Interval> {
    // for each mapping we need to get the list of intersections and non intersections
    let mut curr_intervals = intervals.clone();
    let mut mapped_intervals = Vec::new();

    for mapping in mappings {
        let mut next_intervals: HashSet<Interval> = HashSet::new();
        for interval in curr_intervals {
            let (intersection, untouched) = interval.overlaps(&mapping.source_interval);
            if let Some(mapped) = intersection {
                mapped_intervals.push(map_interval(mapped, mapping));
                untouched.iter().for_each(|leftover| {
                    next_intervals.insert(leftover.clone());
                });
            } else {
                next_intervals.insert(interval.clone());
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

#[derive(Eq, PartialEq, Debug, Clone, Hash, PartialOrd)]
struct Interval {
    start: usize,
    // end is exclusive
    end: usize,
}

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.start == other.start {
            return std::cmp::Ordering::Equal;
        }
        match self.start < other.start {
            true => std::cmp::Ordering::Less,
            false => std::cmp::Ordering::Greater,
        }
    }
}

impl Interval {
    fn new(start: usize, end: usize) -> Interval {
        return Interval { start, end };
    }

    fn intersect(&self, other: &Interval) -> Option<Interval> {
        // if other.start < self.start {
        //     return other.intersect(self);
        // }

        // 1: a b c d
        // 2: a c b d
        // 3: a c d b
        // 4: c d a b
        // 5: c a d b
        // 6: c a b d
        if self.end <= other.start || other.end <= self.start {
            // a b c d
            // c d a b
            return None;
        } else if self.start <= other.start && other.start < self.end && self.end <= other.end {
            // a c b d
            return Some(Interval::new(other.start, self.end));
        } else if self.start <= other.start && other.end <= self.end {
            // a c d b
            return Some(Interval::new(other.start, other.end));
        } else if other.start <= self.start && self.start < other.end && other.end <= self.end {
            // c a d b
            return Some(Interval::new(self.start, other.end));
        } else {
            // c a b d
            return Some(Interval::new(self.start, self.end));
        }
    }

    fn overlaps(&self, other: &Interval) -> (Option<Interval>, Vec<Interval>) {
        let intersection = self.intersect(other);
        // 1: a b c d
        // 4: c d a b
        // 2: a c b d
        // 3: a c d b
        // 5: c a d b
        // 6: c a b d

        if intersection == None {
            return (None, vec![self.clone()]);
        }
        // Either we have a b c d -> covered
        // self = [A, B)
        // intersectin = [C, D)
        let it = intersection.unwrap();
        // we know that intersection is contained in self
        //
        // What we want to get is whether self is before it at all
        // or if it is after
        // and from the end of the it to the end of other.
        // this is assuming that self < other
        let mut remaining = Vec::new();
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

        return (Some(it), remaining);
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

        let mut disjoint_1 = first.overlaps(&second);
        let mut disjoint_2 = second.overlaps(&first);

        let expected = vec![first, second].sort();
        assert_eq!(disjoint_1.0, disjoint_2.0);
        assert_eq!(disjoint_1.1.sort(), disjoint_2.1.sort());

        assert_eq!(None, disjoint_1.0);
        assert_eq!(
            expected,
            disjoint_1.1.into_iter().collect::<Vec<Interval>>().sort()
        );
    }

    #[test]
    fn test_overlaps_intersection() {
        let first = Interval::new(1, 4);
        let second = Interval::new(3, 5);

        let mut disjoint_1 = first.overlaps(&second);
        let mut disjoint_2 = second.overlaps(&first);

        let expected = vec![Interval::new(1, 3), Interval::new(4, 5)].sort();
        assert_eq!(disjoint_1.0, disjoint_2.0);
        assert_eq!(disjoint_1.1.sort(), disjoint_2.1.sort());

        assert_eq!(Some(Interval::new(3, 4)), disjoint_1.0);
        assert_eq!(
            expected,
            disjoint_1.1.into_iter().collect::<Vec<Interval>>().sort()
        );
    }

    #[test]
    fn test_overlaps_intersection_same_end() {
        let first = Interval::new(1, 4);
        let second = Interval::new(3, 4);

        let mut disjoint_1 = first.overlaps(&second);
        let mut disjoint_2 = second.overlaps(&first);

        let expected = vec![Interval::new(1, 3)].sort();
        assert_eq!(disjoint_1.0, disjoint_2.0);
        assert_eq!(disjoint_1.1.sort(), disjoint_2.1.sort());

        assert_eq!(Some(Interval::new(3, 4)), disjoint_1.0);
        assert_eq!(
            expected,
            disjoint_1.1.into_iter().collect::<Vec<Interval>>().sort()
        );
    }

    #[test]
    fn test_overlaps_contains() {
        let first = Interval::new(2, 4);
        let second = Interval::new(1, 5);

        let mut overlap_1 = first.overlaps(&second);
        let mut overlap_2 = second.overlaps(&first);

        let expected = vec![Interval::new(1, 2), Interval::new(4, 5)].sort();
        assert_eq!(overlap_1.0, overlap_2.0);
        assert_eq!(overlap_1.1.sort(), overlap_2.1.sort());

        assert_eq!(Some(Interval::new(2, 4)), overlap_1.0);
        assert_eq!(expected, overlap_1.1.sort());
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
