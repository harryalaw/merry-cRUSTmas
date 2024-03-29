use hashbrown::HashMap;
use rayon::prelude::*;

#[tracing::instrument]
pub fn process(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    lines
        .into_par_iter()
        .flat_map(|line| line.split_once(' '))
        .map(|(chars, numbers)| {
            let springs = repeat_five(chars, '?').chars().collect();
            let damaged = repeat_five(numbers, ',')
                .split(',')
                .map(|x| x.parse().expect("It's a number"))
                .collect();

            Record::new(springs, damaged).check()
        }).sum()
}

struct Record {
    springs: Vec<char>,
    damaged: Vec<u8>,
    spring_len: u8,
    damaged_len: u8,
}

impl Record {
    fn new(springs: Vec<char>, damaged: Vec<u8>) -> Record {
        let spring_len = springs.len() as u8;
        let damaged_len = damaged.len() as u8;

        Record {
            springs,
            damaged,
            spring_len,
            damaged_len,
        }
    }

    fn check(&self) -> usize {
        let remaining = self.damaged.iter().sum();
        let mut cache: HashMap<(u8, u8, u8), usize> = HashMap::new();
        self.count(0, 0, 0, remaining, &mut cache)
    }

    fn count(&self, spring_idx: u8, damaged_idx: u8, seen: u8, remaining: u8, cache: &mut HashMap<(u8,u8,u8), usize>) -> usize {
        // not enough space for the rest
        if self.spring_len - spring_idx + seen < remaining {
            return 0;
        }

        if let Some(prev) = cache.get(&(spring_idx, damaged_idx, seen)) {
            return *prev;
        }

        if spring_idx == self.spring_len {
            if damaged_idx == self.damaged_len && seen == 0
                || damaged_idx == self.damaged_len - 1 && seen == self.damaged[damaged_idx as usize]
            {
                return 1;
            } else {
                return 0;
            }
        }

        let mut total = 0;
        match self.springs[spring_idx as usize] {
            '.' => {
                if seen == 0 {
                    total += self.count(spring_idx + 1, damaged_idx, 0, remaining, cache);
                } else if damaged_idx < self.damaged_len
                    && seen == self.damaged[damaged_idx as usize]
                {
                    total += self.count(
                        spring_idx + 1,
                        damaged_idx + 1,
                        0,
                        remaining - self.damaged[damaged_idx as usize],
                        cache,
                    );
                };
            }
            '#' => {
                if damaged_idx < self.damaged_len && seen < self.damaged[damaged_idx as usize] {
                    total += self.count(spring_idx + 1, damaged_idx, seen + 1, remaining, cache);
                }
            }
            '?' => {
                // treat as damaged
                if damaged_idx < self.damaged_len && seen < self.damaged[damaged_idx as usize] {
                    total += self.count(spring_idx + 1, damaged_idx, seen + 1, remaining, cache);
                }

                // treat as operational
                if seen == 0 {
                    total += self.count(spring_idx + 1, damaged_idx, 0, remaining, cache);
                } else if damaged_idx < self.damaged_len
                    && seen == self.damaged[damaged_idx as usize]
                {
                    total += self.count(
                        spring_idx + 1,
                        damaged_idx + 1,
                        0,
                        remaining - self.damaged[damaged_idx as usize],
                        cache
                    );
                };
            }
            _ => panic!("Invalid char {}", self.springs[spring_idx as usize]),
        }
        cache.insert((spring_idx, damaged_idx, seen), total);
        total
    }
}

fn repeat_five(input: &str, joining_char: char) -> String {
    [input; 5].join(&joining_char.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(525152, process(input));
    }

    #[test]
    fn test_input_cases_1() {
        let input = "???.### 1,1,3";
        assert_eq!(1, process(input));
    }

    #[test]
    fn test_input_cases_2() {
        let input = ".??..??...?##. 1,1,3";
        assert_eq!(16384, process(input));
    }

    #[test]
    fn test_input_cases_3() {
        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        assert_eq!(1, process(input));
    }

    #[test]
    fn test_input_cases_4() {
        let input = "????.#...#... 4,1,1";
        assert_eq!(16, process(input));
    }

    #[test]
    fn test_input_cases_5() {
        let input = "????.######..#####. 1,6,5";
        assert_eq!(2500, process(input));
    }

    #[test]
    fn test_input_cases_6() {
        let input = "?###???????? 3,2,1";
        assert_eq!(506250, process(input));
    }
}
