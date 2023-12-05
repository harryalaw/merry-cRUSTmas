use std::collections::{HashMap, HashSet};

#[tracing::instrument]
pub fn process(input: &str) -> usize {
    let mut counts: HashMap<usize, usize> = HashMap::new();

    let lines = input.lines().collect::<Vec<&str>>();
    for i in 0..lines.len() {
        counts.insert(i + 1, 1);
    }

    for i in 0..lines.len() {
        let winners = parse_card(lines.get(i).expect("We can index this"));
        let mut new_map: HashMap<usize, usize> = counts.clone();

        let previous_count = counts.get(&(i + 1)).unwrap_or(&1);

        for j in i + 2..i + winners + 2 {
            let previous_amount = counts.get(&j).unwrap_or(&1);
            new_map.insert(j, previous_amount + previous_count);
        }

        counts = new_map
    }

    counts.into_iter().fold(0, |acc, (_key, val)| acc + val)
}

fn parse_card(card: &str) -> usize {
    let parts = card.split_once(": ").expect("It has a colon");

    let numbers = parts.1.split_once(" | ").expect("It has a pipe");

    let winner_numbers = numbers
        .0
        .split_ascii_whitespace()
        .flat_map(|x| x.parse::<usize>())
        .collect::<HashSet<usize>>();
    let my_numbers = numbers
        .1
        .split_ascii_whitespace()
        .flat_map(|x| x.parse::<usize>())
        .collect::<HashSet<usize>>();

    winner_numbers.intersection(&my_numbers).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(30, process(input));
    }
}
