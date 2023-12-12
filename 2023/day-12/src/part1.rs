use rayon::prelude::*;

#[tracing::instrument]
pub fn process(_input: &str) -> usize {
    let mut records = parse_input(_input);

    records.par_iter_mut().map(|record| record.check()).sum()
}

struct Record {
    springs: Vec<char>,
    damaged: Vec<usize>,
}

impl Record {
    fn check(&mut self) -> usize {
        let remaining = self.damaged.iter().sum();
        self.count(0, 0, 0, remaining)
    }

    fn count(
        &mut self,
        spring_idx: usize,
        damaged_idx: usize,
        seen: usize,
        remaining: usize,
    ) -> usize {
        // not enough space for the rest
        if self.springs.len() - spring_idx + seen < remaining {
            return 0;
        }

        if spring_idx == self.springs.len() {
            if damaged_idx == self.damaged.len() && seen == 0
                || damaged_idx == self.damaged.len() - 1 && seen == self.damaged[damaged_idx]
            {
                return 1;
            } else {
                return 0;
            }
        }

        let mut total = 0;
        match self.springs[spring_idx] {
            '.' => {
                if seen == 0 {
                    total += self.count(spring_idx + 1, damaged_idx, 0, remaining);
                } else if damaged_idx < self.damaged.len() && seen == self.damaged[damaged_idx] {
                    total += self.count(
                        spring_idx + 1,
                        damaged_idx + 1,
                        0,
                        remaining - self.damaged[damaged_idx],
                    );
                };
            }
            '#' => {
                if damaged_idx < self.damaged.len() && seen < self.damaged[damaged_idx] {
                    total += self.count(spring_idx + 1, damaged_idx, seen + 1, remaining);
                }
            }
            '?' => {
                // treat as damaged
                if damaged_idx < self.damaged.len() && seen < self.damaged[damaged_idx] {
                    total += self.count(spring_idx + 1, damaged_idx, seen + 1, remaining);
                }

                // treat as operational
                if seen == 0 {
                    total += self.count(spring_idx + 1, damaged_idx, 0, remaining);
                } else if damaged_idx < self.damaged.len() && seen == self.damaged[damaged_idx] {
                    total += self.count(
                        spring_idx + 1,
                        damaged_idx + 1,
                        0,
                        remaining - self.damaged[damaged_idx],
                    );
                };
            }
            _ => panic!("Invalid char {}", self.springs[spring_idx]),
        }
        total
    }
}

fn parse_input(input: &str) -> Vec<Record> {
    input
        .lines()
        .flat_map(|line| line.split_once(' '))
        .map(|(chars, numbers)| {
            let springs = chars.chars().collect();
            let damaged = numbers
                .split(',')
                .map(|x| x.parse().expect("It's a number"))
                .collect();

            Record { springs, damaged }
        })
        .collect()
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
        assert_eq!(21, process(input));
    }

    #[test]
    fn test_input_cases_1() {
        let input = "???.### 1,1,3";
        assert_eq!(1, process(input));
    }

    #[test]
    fn test_input_cases_2() {
        let input = ".??..??...?##. 1,1,3";
        assert_eq!(4, process(input));
    }

    #[test]
    fn test_input_cases_3() {
        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        assert_eq!(1, process(input));
    }

    #[test]
    fn test_input_cases_4() {
        let input = "????.#...#... 4,1,1";
        assert_eq!(1, process(input));
    }

    #[test]
    fn test_input_cases_5() {
        let input = "????.######..#####. 1,6,5";
        assert_eq!(4, process(input));
    }

    #[test]
    fn test_input_cases_6() {
        let input = "?###???????? 3,2,1";
        assert_eq!(10, process(input));
    }
}
