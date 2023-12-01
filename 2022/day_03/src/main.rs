use itertools::izip;
use std::iter::zip;

fn main() {
    do_part1();
    println!();
    do_part2();
}

fn do_part1() {
    println!("Test 1: {}", part1(include_str!("../test.txt")));
    println!("Part 1: {}", part1(include_str!("../input.txt")));
}

fn do_part2() {
    println!("Test 2: {}", part2(include_str!("../test.txt")));
    println!("Part 2: {}", part2(include_str!("../input.txt")));
}

fn part1(input: &str) -> usize {
    return input
        .lines()
        .map(|line| WordPair::from(line).get_points())
        .sum();
}

fn part2(input: &str) -> usize {
    return input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|group| WordGroup::from((group[0], group[1], group[2])).get_points())
        .sum();
}

#[derive(Debug)]
struct Letters {
    letters: [bool; 52],
}

impl From<&str> for Letters {
    fn from(s: &str) -> Letters {
        let letters = s.bytes().into_iter().fold([false; 52], |mut acc, letter| {
            if letter > 96 {
                acc[(letter - 97) as usize] = true;
            } else {
                acc[(letter - 65 + 26) as usize] = true;
            }
            return acc;
        });

        return Letters { letters };
    }
}

#[derive(Debug)]
struct WordPair {
    first_letters: Letters,
    second_letters: Letters,
}

impl From<&str> for WordPair {
    fn from(s: &str) -> Self {
        let (first_word, second_word) = s.split_at(s.len() / 2);

        let first_letters = Letters::from(first_word);
        let second_letters = Letters::from(second_word);

        return WordPair {
            first_letters,
            second_letters,
        };
    }
}

impl GetPoints for WordPair {
    fn get_points(&self) -> usize {
        let first_iter = self.first_letters.letters.iter();
        let second_iter = self.second_letters.letters.iter();
        let combined: Vec<bool> = zip(first_iter, second_iter)
            .map(|(a, b)| *a && *b)
            .collect();
        return combined.iter().enumerate().fold(0, |acc, (i, val)| {
            if *val {
                return i + 1;
            }
            return acc;
        });
    }
}

struct WordGroup {
    first_word: Letters,
    second_word: Letters,
    third_word: Letters,
}

impl From<(&str, &str, &str)> for WordGroup {
    fn from(s: (&str, &str, &str)) -> Self {
        return WordGroup {
            first_word: Letters::from(s.0),
            second_word: Letters::from(s.1),
            third_word: Letters::from(s.2),
        };
    }
}

impl GetPoints for WordGroup {
    fn get_points(&self) -> usize {
        let first_iter = self.first_word.letters.iter();
        let second_iter = self.second_word.letters.iter();
        let third_iter = self.third_word.letters.iter();
        let combined: Vec<bool> = izip!(first_iter, second_iter, third_iter)
            .map(|(a, b, c)| *a && *b && *c)
            .collect();
        return combined.iter().enumerate().fold(0, |acc, (i, val)| {
            if *val {
                return i + 1;
            }
            return acc;
        });
    }
}

trait GetPoints {
    fn get_points(&self) -> usize;
}
