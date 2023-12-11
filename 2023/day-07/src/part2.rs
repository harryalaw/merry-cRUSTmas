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
        let mut highest = (Rank::Ace, 0);
        let mut second_highest = (Rank::Ace, 0);

        let mut rank_counts: [usize; 13] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        for card in cards {
            rank_counts[card.index()] += 1;
        }
        let joker_counts = rank_counts[Rank::Joker.index()];

        for card in Rank::iterator() {
            if card == Rank::Joker {
                continue;
            }
            let value = rank_counts[card.index()];
            if value > highest.1 {
                second_highest = highest;
                highest = (card, value);
            } else if value > second_highest.1 {
                second_highest = (card, value);
            }
        }
        highest.1 += joker_counts;

        match (highest.1, second_highest.1) {
            (5, 0) => HandType::FiveOfAKind,
            (4, 1) => HandType::FourOfAKind,
            (3, 2) => HandType::FullHouse,
            (3, 1) => HandType::ThreeOfAKind,
            (2, 2) => HandType::TwoPair,
            (2, 1) => HandType::OnePair,
            (1, 1) => HandType::HighCard,
            (_a, _b) => panic!(
                "we didn't think of this one {} {}\n{:?}",
                _a, _b, &rank_counts
            ),
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
            'J' => Rank::Joker,
            'T' => Rank::Ten,
            '9' => Rank::Nine,
            '8' => Rank::Eight,
            '7' => Rank::Seven,
            '6' => Rank::Six,
            '5' => Rank::Five,
            '4' => Rank::Four,
            '3' => Rank::Three,
            '2' => Rank::Two,
            _ => panic!("Invalid character"),
        }
    }

    fn index(&self) -> usize {
        match self {
            Rank::Ace => 0,
            Rank::Two => 1,
            Rank::Three => 2,
            Rank::Four => 3,
            Rank::Five => 4,
            Rank::Six => 5,
            Rank::Seven => 6,
            Rank::Eight => 7,
            Rank::Nine => 8,
            Rank::Ten => 9,
            Rank::Joker => 10,
            Rank::Queen => 11,
            Rank::King => 12,
        }
    }

    pub fn iterator() -> impl Iterator<Item = Rank> {
        [
            Rank::Ace,
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Joker,
            Rank::Queen,
            Rank::King,
        ]
        .iter()
        .copied()
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
