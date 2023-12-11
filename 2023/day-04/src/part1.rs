#[tracing::instrument]
pub fn process(input: &str) -> usize {
    input.lines().map(parse_card).sum()
}

fn parse_card(card: &str) -> usize {
    let parts = card.split_once(": ").expect("It has a colon");

    let numbers = parts.1.split_once(" | ").expect("It has a pipe");

    let winner_numbers = numbers
        .0
        .split_ascii_whitespace()
        .flat_map(|x| x.parse::<usize>())
        .collect::<Vec<usize>>();
    let my_numbers = numbers
        .1
        .split_ascii_whitespace()
        .flat_map(|x| x.parse::<usize>())
        .collect::<Vec<usize>>();

    intersect(&winner_numbers, &my_numbers)
        .iter()
        .fold(1, |acc, _curr| acc * 2)
        >> 1
}

fn intersect(vec1: &[usize], vec2: &[usize]) -> Vec<usize> {
    vec1.iter().filter(|x| vec2.contains(x)).copied().collect()
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
        assert_eq!(13, process(input));
    }
}
