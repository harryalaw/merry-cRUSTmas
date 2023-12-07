use std::collections::HashMap;

#[tracing::instrument]
pub fn process(_input: &str) -> usize {
    let mut hands = parse_hands(_input);
    hands.sort();

    hands
        .iter()
        .rev()
        .enumerate()
        .map(|(rank, hand)| hand.bid * (rank + 1))
        .sum::<usize>()
}

fn parse_hands(input: &str) -> Vec<Hand> {
    input.lines().map(parse_hand).collect()
}

fn parse_hand(input: &str) -> Hand {
    let parts = input.split_once(' ').expect("a space");
    let bid = parts.1.parse::<usize>().expect("It's a number");
    let cards: Vec<Rank> = parts.0.chars().map(Rank::new).collect();
    let hand_type = HandType::new(&cards);

    Hand {
        cards,
        hand_type,
        bid,
    }
}

#[derive(Eq, PartialEq, PartialOrd, Debug, Ord, Hash, Copy, Clone)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn new(cards: &[Rank]) -> HandType {
        let joker_count = cards.iter().filter(|x| **x == Rank::Joker).count();
        if joker_count == 5 {
            return HandType::FiveOfAKind;
        }

        let mut rank_to_counts: HashMap<Rank, usize> = HashMap::new();
        for card in cards {
            if card == &Rank::Joker {
                continue;
            }
            let previous = rank_to_counts.get(card).unwrap_or(&0);
            rank_to_counts.insert(*card, previous + 1);
        }

        if let Some(highest_value) =
            rank_to_counts
                .iter()
                .fold(None, |best: Option<(&Rank, &usize)>, curr| {
                    if best.is_none() || curr.1 > best.unwrap().1 {
                        Some(curr)
                    } else {
                        best
                    }
                })
        {
            rank_to_counts.insert(*highest_value.0, *highest_value.1 + joker_count);
        };

        let mut counts_to_rank: HashMap<usize, Vec<Rank>> = HashMap::new();
        for x in rank_to_counts.iter() {
            let previous = counts_to_rank.get(x.1);

            let mut next = match previous {
                Some(vec) => vec.clone(),
                None => Vec::new(),
            };

            next.push(*x.0);
            counts_to_rank.insert(*x.1, next);
        }

        if counts_to_rank.contains_key(&5) {
            HandType::FiveOfAKind
        } else if counts_to_rank.contains_key(&4) {
            HandType::FourOfAKind
        } else if counts_to_rank.contains_key(&3) {
            if counts_to_rank.contains_key(&2) {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        } else if counts_to_rank.contains_key(&2) {
            let highs = counts_to_rank.get(&2).unwrap();
            if highs.len() == 2 {
                HandType::TwoPair
            } else {
                HandType::OnePair
            }
        } else {
            HandType::HighCard
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Hand {
    cards: Vec<Rank>,
    hand_type: HandType,
    bid: usize,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => self.cards.cmp(&other.cards),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
enum Rank {
    Ace,
    King,
    Queen,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl Rank {
    fn new(c: char) -> Rank {
        match c {
            'A' => Rank::Ace,
            'K' => Rank::King,
            'Q' => Rank::Queen,
            'T' => Rank::Ten,
            '9' => Rank::Nine,
            '8' => Rank::Eight,
            '7' => Rank::Seven,
            '6' => Rank::Six,
            '5' => Rank::Five,
            '4' => Rank::Four,
            '3' => Rank::Three,
            '2' => Rank::Two,
            'J' => Rank::Joker,
            _ => panic!("Invalid character"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(5905, process(input));
    }
}
